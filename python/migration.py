import datetime
import dateutil.parser
from influxdb import InfluxDBClient

THIRD_DAY_NAME = "third_day_completeness"
SEVENTH_DAY_NAME = "seventh_day_completeness"
WEEK = 7

def migrations():
    db_info = {"url":"localhost", "db":"marketplace"}
    db_credential = {"username":"", "password":""}

    influx_client = InfluxDBClient(db_info['url'], 8086, db_credential['username'], db_credential['password'], db_info['db'])
    dbs = influx_client.get_list_database()

    if dbs.count({"name": db_info['db']}) == 0:
        influx_client.create_database(db_info['db'])

    print "Migration beginning..."

    third_day_back = copy_data_to_new(influx_client, THIRD_DAY_NAME)
    seventh_day_back = copy_data_to_new(influx_client, SEVENTH_DAY_NAME)

    drop_coll(influx_client, THIRD_DAY_NAME)
    drop_coll(influx_client, SEVENTH_DAY_NAME)

    migrations_days(influx_client, 3, THIRD_DAY_NAME, third_day_back)
    migrations_days(influx_client, 7, SEVENTH_DAY_NAME, seventh_day_back)

    drop_coll(influx_client, third_day_back)
    drop_coll(influx_client, seventh_day_back)

    print "End Migrations"

def copy_data_to_new(client, coll_name):
    new_coll_name = coll_name + '_back'

    client.query('SELECT * INTO {0} FROM {1};'.format(new_coll_name, coll_name))

    print "Copy data to new collection %s" % new_coll_name

    # check
    old_result = client.query('select * from {0};'.format(coll_name))
    old_list = list(old_result.get_points(measurement=coll_name))

    new_result = client.query('select * from {0};'.format(new_coll_name))
    new_list = list(new_result.get_points(measurement=new_coll_name))

    if len(old_list) != len(new_list):
        raise AssertionError("Data copy failed. migration stop, no recored change.")
    else:
        print "%s length == %s length" % (new_coll_name, coll_name)

    if cmp(old_list, new_list) != 0:
        raise AssertionError("Data copy failed. migration stop, no recored change.")
    else:
        print "%s == %s " % (new_coll_name, coll_name)

    return new_coll_name


def migrations_days(client, time_window_num, coll_name, back_name):
    print "Migration: %s to %s" % (back_name, coll_name)

    last_time = datetime.datetime.now().utcnow() + (datetime.timedelta(days=-(WEEK - time_window_num)))
    time_str = datetime.datetime.strftime(last_time, "%Y-%m-%dT%H:%M:%SZ")

    result = client.query("select * from {0};".format(back_name))
    points = list(result.get_points(measurement=back_name))

    index = 1;
    time_window = datetime.timedelta(days=-time_window_num)
    for item in points:
        new_time = dateutil.parser.parse(item['time']) + time_window

        point = {
            "measurement": coll_name,
            "time": new_time,
            "fields": {
                "value": item['value']
            }
        }

        success = client.write_points([point])
        if not success:
            raise AssertionError("fail to write points")

        print "%s: %s -to- %s: %s" % (index, item['time'], new_time, item['value'])
        index += 1


def drop_coll(client, coll_name):
    client.query('DROP MEASUREMENT {0};'.format(coll_name))
    print "DROP MEASUREMENT %s" % coll_name


if __name__ == '__main__':

    migrations()
