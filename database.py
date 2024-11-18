import mysql.connector

class DataBase:
    def __init__(self):
        with open('.dbpass', 'r') as file:
            password=file.read()[:16]
        self.user='root'
        self.password=password
        self.host='127.0.0.1'
        self.database='hunting'

    def request(self, query, params):
        cnx = mysql.connector.connect(
            user=self.user,
            password=self.password,
            host=self.host,
            database=self.database
        )
        cursor = cnx.cursor()
        cursor.execute(query, params)
        data=cursor.fetchall()
        cnx.commit()
        cursor.close()
        cnx.close()
        return data
