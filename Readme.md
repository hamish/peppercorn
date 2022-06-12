I'm working my way through the zero2rust book.
https://www.lpalmieri.com/posts/2020-05-24-zero-to-production-0-foreword/

Reusable test:
NOW=`date +%s`; curl -i -X POST -d "email=thomas_mann_${NOW}@hotmail.com&name=Tom"     http://127.0.0.1:8000/subscriptions