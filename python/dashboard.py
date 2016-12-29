from pymongo import MongoClient
from bson.objectid import ObjectId

import argparse
import threading
import time
import datetime
from time import sleep

import new_user


def installRegister(count):
    print "Begin install register record -----------------------------"

    MONGO_URL = "mongodb://localhost:27017"
    MONGO_DB = "mp"

    client = MongoClient(MONGO_URL)
    db = client[MONGO_DB]

    newId = db.post_login.insert_one({
        "preLoginId" : ObjectId("5864b51868514d1ee1255b4e"),
        "userId" : ObjectId("58648ac968514d31c61e024a"),
        "isRegisterUser" : True,
        "duration" : 8127169083,
        "createdTime" : datetime.datetime.now().utcnow()}).inserted_id

    print "insert_one record, ID: %s" % newId



if __name__ == '__main__':

    work_types = ["setup", "drop"]

    parser = argparse.ArgumentParser()
    parser.add_argument("type", help="work type", choices=work_types)
    args = parser.parse_args()

    print "work on %s" % args.type

    threads = []
    new_user_thread = new_user.NewUser(5, 2)
    new_user_thread.setDaemon(True)
    new_user_thread.start()
    
    while True:
        for t in threads:
            t.stop()
            t.join()
        pass
