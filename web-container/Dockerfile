FROM python:3.12

WORKDIR /code

COPY ./app /code/app

RUN pip install --no-cache-dir --upgrade -r /code/app/requirements.txt

CMD ["gunicorn", "--conf", "app/gunicorn_conf.py", "--bind", "0.0.0.0:80", "app.main:app"]
