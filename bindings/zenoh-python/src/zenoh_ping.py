#
# Copyright (c) 2022 ZettaScale Technology
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
#

import time
import argparse
import json
import zenoh
from zenoh import Value
from threading import Condition
import math

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser()
parser.add_argument('--mode', '-m', dest='mode',
                    choices=['peer', 'client'],
                    type=str,
                    help='The zenoh session mode.')
parser.add_argument('--connect', '-e', dest='connect',
                    metavar='ENDPOINT',
                    action='append',
                    type=str,
                    help='Endpoints to connect to.')
parser.add_argument('--listen', '-l', dest='listen',
                    metavar='ENDPOINT',
                    action='append',
                    type=str,
                    help='Endpoints to listen on.')
parser.add_argument('--size', '-s', dest='payload_size',
                    default=8,
                    type=int,
                    help='The size of the paylaod.')
parser.add_argument('--interval', '-i', dest='interval',
                    default=0.1,
                    type=float,
                    help='The size of the paylaod.')
parser.add_argument('--config', '-c', dest='config',
                    metavar='FILE',
                    type=str,
                    help='A configuration file.')

args = parser.parse_args()
conf = zenoh.Config.from_file(
    args.config) if args.config is not None else zenoh.Config()
if args.mode is not None:
    conf.insert_json5(zenoh.config.MODE_KEY, json.dumps(args.mode))
if args.connect is not None:
    conf.insert_json5(zenoh.config.CONNECT_KEY, json.dumps(args.connect))
if args.listen is not None:
    conf.insert_json5(zenoh.config.LISTEN_KEY, json.dumps(args.listen))
size = args.payload_size


start = 0
end = 0
cond = Condition()

def sub_callback(_):
    global cond
    cond.acquire()
    cond.notify()
    cond.release()

def main():
    global start, end, cond

    # initiate logging
    zenoh.init_logger()
    session = zenoh.open(conf)

    data = bytearray()
    for i in range(0, size):
        data.append(i % 10)
    data = Value(bytes(data))

    pub = session.declare_publisher('test/ping')
    _sub = session.declare_subscriber('test/pong', sub_callback)

    cond.acquire()
    while True:
        time.sleep(args.interval)
        start = time.time()
        pub.put(data)
        if not cond.wait(1):
            continue
        end = time.time()
        print(f"{size},{math.floor(((end - start) / 2)*1000000)}")

    session.close()
main()
