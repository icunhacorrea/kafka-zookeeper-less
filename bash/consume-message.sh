#!/bin/bash


docker run --network=host -it --rm \
        edenhill/kcat:1.7.1 \
           -b 0.0.0.0:9093 \
           -C \
           -f '\nKey (%K bytes): %k\t\nValue (%S bytes): %s\n\Partition: %p\tOffset: %o\n--\n' \
           -t teste-topic
