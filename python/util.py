# -*- coding: utf8 -*-
import pymongo

import config


def get_db():
    return pymongo.MongoClient(config.MONGO_URL)[config.MONGO_DB]


def get_users(limit):
    users = get_db()["user"].find({}).limit(limit)

    print "Find user count: %s" % users.count()

    return users
