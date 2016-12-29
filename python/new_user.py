
import threading
import time
import datetime

import pymongo
from bson.objectid import ObjectId

MONGO_URL = "mongodb://localhost:27017"
MONGO_DB = "mp"

class NewUser(threading.Thread):
    def __init__(self, count, interval):
        threading.Thread.__init__(self)

        self.count = count # record count
        self.interval = interval
        self.db = None
        self.users = []
        self.index = 0

    def run(self):
        print "New User thread startting..."

        client = pymongo.MongoClient(MONGO_URL);
        self.db = client[MONGO_DB]
        self.get_users()

        if self.count > self.users.count():
            self.count = self.users.count()

        while self.index < self.count:
            user = self.users[self.index]
            print "%s ------------------------------------------" % self.index

            print "Create per-login record, UserId: %s" % user['_id']

            print "Create post-login record, UserId: %s" % user['_id']

            self.index += 1
            time.sleep(self.interval)

    def stop(self):
        self.index = self.count
        print "New User thread done."


    def get_users(self):
        self.users = self.db["user"].find({}).limit(self.count)

        print "find user count: %s" % self.users.count()


    def create_per_login(self, user):

        return None

    def create_post_login(self, user):

        return None
