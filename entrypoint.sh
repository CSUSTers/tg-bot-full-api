#!/bin/sh

if [ ! -d "/data" ]; then
  mkdir /data
fi

if [ -n "$TELEGRAM_LOCAL_MODE" ]; then
  /telegram-bot-api --local -t /tmp -d /data &
else
  /telegram-bot-api -t /tmp -d /data &
fi

/tg-bot-full-api &

wait
