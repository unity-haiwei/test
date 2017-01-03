# -*- coding: utf8 -*-

import threading
import time
import datetime
import uuid

import pymongo
from bson.objectid import ObjectId

import config

LOGIN_OLD = "old"
LOGIN_EXISTS_USER = "exists_user"
LOGIN_REGISTER = "register"

class LoginThread(threading.Thread):

    def __init__(self, interval):
        threading.Thread.__init__(self)

        self.count = 0
        self.interval = interval
        self.created_time = datetime.datetime.now().utcnow()
        self.db = None
        self.users = []
        self.index = 0
        self.data_list = [
            [LOGIN_OLD, LOGIN_REGISTER],
            [LOGIN_EXISTS_USER, LOGIN_REGISTER],
            [LOGIN_OLD, LOGIN_EXISTS_USER],
            [LOGIN_REGISTER]
        ]


    def run(self):
        print "User Login thread running... -- %s" % self.created_time

        self.count = self.get_data_list_len()
        self.db = pymongo.MongoClient(config.MONGO_URL)[config.MONGO_DB]
        self.get_users()

        if self.count > self.users.count():
            self.count = self.users.count()

        print "Create %s records, Interval time: %ss" % (self.count, self.interval)

        # created_time move left
        self.created_time = self.created_time - datetime.timedelta(seconds=self.count * self.interval)

        for i in range(self.data_list.__len__()):

            for item in self.data_list[i]:
                user = self.users[self.index]

                print "----------------------------------------------"
                print "Index: %s  UserId: %s" % (self.index + 1, user["_id"])

                if item == LOGIN_OLD:
                    self.create_post_login_old(user, self.create_pre_login())
                if item == LOGIN_EXISTS_USER:
                    self.create_post_login_exists(user, self.create_pre_login())
                if item == LOGIN_REGISTER:
                    self.create_post_login_register(user, self.create_pre_login())

                self.index += 1

            self.created_time = self.created_time + datetime.timedelta(seconds=self.interval)
            # time.sleep(self.interval)

        print "User Login thread done"


    def stop(self):
        print "User Login thread stop."


    def get_data_list_len(self):
        result = 0
        for arr in self.data_list:
            result += arr.__len__()

        return result


    def get_users(self):
        self.users = self.db["user"].find({}).limit(self.count)

        print "Find user count: %s" % self.users.count()


    # Result: record
    def create_pre_login(self):
        record = {
            "path" : "/auth/callback",
            "state" : uuid.uuid1().__str__(),
            "createdTime" : self.created_time
        }
        id = self.db.pre_login.insert_one(record).inserted_id

        print "Create per-login record, Id: %s" % id

        return id


    # old login data
    def create_post_login_old(self, user, pre_login_id):
        record = {
            "preLoginId" : pre_login_id,
            "userId" : user['_id'],
            "duration" : 5005225321,
            "createdTime" : self.created_time
        }
        id = self.db.post_login.insert_one(record).inserted_id

        print "Create post-login--old record, UserId: %s" % user['_id']


    # exists user login
    def create_post_login_exists(self, user, pre_login_id):
        record = {
            "preLoginId" : pre_login_id,
            "userId" : user['_id'],
            "isRegisterUser" : False,
            "duration" : 5005225321,
            "createdTime" : self.created_time
        }
        id = self.db.post_login.insert_one(record).inserted_id

        print "Create post-login--exists record, UserId: %s" % user['_id']


    # register user login
    def create_post_login_register(self, user, pre_login_id):
        record = {
            "preLoginId" : pre_login_id,
            "userId" : user['_id'],
            "isRegisterUser" : True,
            "duration" : 5005225321,
            "createdTime" : self.created_time
        }
        id = self.db.post_login.insert_one(record).inserted_id

        print "Create post-login--register record, UserId: %s" % user['_id']
