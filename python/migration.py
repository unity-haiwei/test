import datetime
import dateutil.parser
from influxdb import InfluxDBClient


def migrations():
    currentInfluxInfo = {"url":"localhost", "db":"marketplace"}
    currentInfluxCredential = {"username":"", "password":""}

    influx_client = InfluxDBClient(
        currentInfluxInfo['url'],
        8086,
        currentInfluxCredential['username'],
        currentInfluxCredential['password'],
        currentInfluxInfo['db'])

    dbs = influx_client.get_list_database()

    if dbs.count({"name": currentInfluxInfo['db']}) == 0:
        influx_client.create_database(name)

    migrations_3_days(influx_client)
    migrations_7_days(influx_client)


def migrations_3_days(client):
    print "Migration 3 days ------------------------------"

    time_window = datetime.timedelta(days=-3)

    result = client.query('select * from third_day_completeness;')
    points = list(result.get_points(measurement='third_day_completeness'))

    for item in points:
        new_time = dateutil.parser.parse(item['time']) + time_window

        print "%s -- %s: %s" % (item['time'], new_time, item['value'])

        point = {
            "measurement": "third_day_completeness",
            "time": new_time,
            "fields": {
                "value": item['value']
            }
        }
        success = client.write_points([point])
        if not success:
            raise AssertionError("fail to write points")


def migrations_7_days(client):
    print "Migration 7 days ------------------------------"

    time_window = datetime.timedelta(days=-7)

    result = client.query('select * from seventh_day_completeness;')
    points = list(result.get_points(measurement='seventh_day_completeness'))

    for item in points:
        new_time = dateutil.parser.parse(item['time']) + time_window

        print "%s -- %s: %s" % (item['time'], new_time, item['value'])

        point = {
            "measurement": "seventh_day_completeness",
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
