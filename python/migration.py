import datetime
import dateutil.parser
from influxdb import InfluxDBClient


def migrations():
    db_info = {"url":"localhost", "db":"marketplace"}
    db_credential = {"username":"", "password":""}

    influx_client = InfluxDBClient(db_info['url'], 8086, db_credential['username'], db_credential['password'], db_info['db'])
    dbs = influx_client.get_list_database()

    if dbs.count({"name": db_info['db']}) == 0:
        influx_client.create_database(db_info['db'])

    migrations_days(influx_client, datetime.timedelta(days=-3), "third_day_completeness")
    migrations_days(influx_client, datetime.timedelta(days=-7), "seventh_day_completeness")


def migrations_days(client, time_window, coll_name):
    print "Migration ------------------------------"

    result = client.query('select * from {0};'.format(coll_name))
    points = list(result.get_points(measurement=coll_name))

    for item in points:
        new_time = dateutil.parser.parse(item['time']) + time_window

        print "%s -- %s: %s" % (item['time'], new_time, item['value'])

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


if __name__ == '__main__':

    migrations()
