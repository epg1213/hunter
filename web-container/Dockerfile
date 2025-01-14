FROM python:3.12

WORKDIR /code

COPY ./app /code/app

RUN pip install --no-cache-dir --upgrade -r /code/app/requirements.txt

ARG SQLPASS
ARG SQLPORT
ARG WEBPORT
ARG IPADDR
ENV SQLPASS=$SQLPASS
ENV SQLPORT=$SQLPORT
ENV WEBPORT=$WEBPORT
ENV IPADDR=$IPADDR

CMD ["gunicorn", "--conf", "app/gunicorn_conf.py", "--bind", "$IPADDR:80", "app.main:app"]
