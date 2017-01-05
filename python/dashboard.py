# -*- coding: utf8 -*-

from pymongo import MongoClient
from bson.objectid import ObjectId

import argparse
import threading
import time
import datetime
from time import sleep

import config
import login


if __name__ == '__main__':

    work_types = ["setup", "drop"]

    parser = argparse.ArgumentParser()
    parser.add_argument("type", help="work type", choices=work_types)
    args = parser.parse_args()

    print "work on %s" % args.type

    threads = []
    login_thread = login.LoginThread(config.TIME_DAYS, 5, 2 * 60)
    login_thread.setDaemon(True)
    threads.append(login_thread)

    for t in threads:
        t.start()

    while True:
        for t in threads:
            if t.isAlive():
                t.join(1)
        pass
