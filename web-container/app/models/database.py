import mysql.connector
from os import listdir, environ
from time import sleep

class DataBase:
    def __init__(self):
        password=environ['SQLPASS']
        self.user='root'
        self.password=password
        self.host='127.0.0.1'
        self.port=environ['SQLPORT']
        self.database='hunting'

    def request(self, query, params, many=False):
        cnx = mysql.connector.connect(
            user=self.user,
            password=self.password,
            host=self.host,
            port=self.port,
            database=self.database
        )
        cursor = cnx.cursor()
        if many:
            cursor.executemany(query, params)
            cnx.commit()
            cursor.close()
            cnx.close()
            return []
        cursor.execute(query, params)
        data=cursor.fetchall()
        cnx.commit()
        cursor.close()
        cnx.close()
        return data
