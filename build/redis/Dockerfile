FROM redis:alpine
COPY redis.conf /etc/redis.conf
RUN mkdir -p /run/redis && chown -R redis:redis /run/redis
CMD [ "redis-server", "/etc/redis.conf" ]
