import mysql.connector
from os import listdir
from time import sleep

class DataBase:
    def __init__(self):
        with open('.dbpass', 'r') as file:
            password=file.read()[:16]
        self.user='root'
        self.password=password
        self.host='127.0.0.1'
        self.port='42601'
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
    
    def runfile(self, filename):
        with open(filename, 'r') as file:
            content=file.read()
        for query in content.split(';')[:-1]:
            self.request(query, ())

if __name__=="__main__":
    db=DataBase()
    for i in range(30):
        try:
            db.request('SHOW TABLES', ())
            break
        except mysql.connector.errors.OperationalError:
            sleep(1)
    if i==29:
        exit("ERROR: Could not connect to database after 30 attempts.")
    for file in listdir('db_setup'):
        db.runfile(f"db_setup/{file}")
