# -*- coding: utf8 -*-

import threading
import time
import datetime
import uuid
import random

import pymongo
from bson.objectid import ObjectId

import config
import util

LOGIN_PREPARE = "prepare"
LOGIN_OLD = "old"
LOGIN_EXISTS_USER = "exists_user"
LOGIN_REGISTER = "register"


class LoginThread(threading.Thread):

    def __init__(self, time_type, time_window, interval):
        threading.Thread.__init__(self)

        self.time_type = time_type
        self.time_window = time_window
        self.interval = interval
        self.created_time = datetime.datetime.now().utcnow()
        self.db = util.get_db()
        self.users = util.get_users(10)
        self.data_list = [
            [LOGIN_REGISTER, LOGIN_EXISTS_USER],
            [LOGIN_EXISTS_USER, LOGIN_EXISTS_USER],
            [LOGIN_PREPARE],
            [LOGIN_EXISTS_USER],
            [LOGIN_REGISTER]
        ]
        self.count = 0

    def run(self):
        print "User Login thread running... -- %s" % datetime.datetime.now().utcnow()

        self.work()


    def stop(self):
        print "User Login thread stop."


    def work(self):
        self.count = self.get_data_list_len()

        if self.count > self.users.count():
            self.count = self.users.count()

        print "Create %s records, Interval time: %ss, Time window: %s" % (self.time_window * self.count, self.interval, self.time_window)

        index = 1
        while index <= self.time_window:
            print "Group: %s ----------------------------------------" % index

            # re-init
            self.created_time -= self.generate_timedelta(1)
            self.generate_data()

            index += 1

        print "User Login thread done"


    def generate_data(self):
        max_count = random.randint(0, self.count)
        index = 0

        for i in range(self.data_list.__len__()):

            for item in self.data_list[i]:
                if index > max_count: return None

                user = self.users[index]

                print "Index: %s , UserId: %s" % (index + 1, user["_id"])

                if item == LOGIN_PREPARE:
                    self.create_pre_login()
                if item == LOGIN_OLD:
                    self.create_post_login_old(user, self.create_pre_login())
                if item == LOGIN_EXISTS_USER:
                    self.create_post_login_exists(user, self.create_pre_login())
                if item == LOGIN_REGISTER:
                    self.create_post_login_register(user, self.create_pre_login())

                index += 1

            self.created_time -= datetime.timedelta(seconds=self.interval)
            # time.sleep(self.interval)


    def generate_timedelta(self, index):
        if self.time_type == config.TIME_DAYS:
            return datetime.timedelta(days=index)
        if self.time_type == config.TIME_HOURS:
            return datetime.timedelta(hours=index)

        return datetime.timedelta(minutes=index)



    def get_data_list_len(self):
        result = 0
        for arr in self.data_list:
            result += arr.__len__()

        return result


    # Result: record
    def create_pre_login(self):
        record = {
            "path" : "/auth/callback",
            "state" : uuid.uuid1().__str__(),
            "createdTime" : self.created_time
        }
        id = self.db.pre_login.insert_one(record).inserted_id

        print "Create per-login."

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

        print "Create post-login--old, CreatedTime: %s" % self.created_time


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

        print "Create post-login--exists, CreatedTime: %s" % self.created_time


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

        print "Create post-login--register, CreatedTime: %s" % self.created_time
