import mysql.connector
from os import listdir, getenv
from time import sleep

class DataBase:
    def __init__(self):
        self.user='root'
        self.password=getenv("SQLPASS", "16-char-PASSWORD")
        self.host=getenv("IPADDR", '192.168.1.1')
        self.port=getenv("SQLPORT", 42601)
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
