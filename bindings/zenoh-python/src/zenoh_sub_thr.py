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

import sys
import time
import argparse
import json
import zenoh
from zenoh import Reliability

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='z_sub_thr',
    description='zenoh throughput sub example')
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
                    metavar='PAYLOAD_SIZE',
                    required=True,
                    type=int,
                    help='Size of each put message in bytes.')
parser.add_argument('--config', '-c', dest='config',
                    metavar='FILE',
                    type=str,
                    help='A configuration file.')

args = parser.parse_args()
conf = zenoh.Config.from_file(args.config) if args.config is not None else zenoh.Config()
if args.mode is not None:
    conf.insert_json5(zenoh.config.MODE_KEY, json.dumps(args.mode))
if args.connect is not None:
    conf.insert_json5(zenoh.config.CONNECT_KEY, json.dumps(args.connect))
if args.listen is not None:
    conf.insert_json5(zenoh.config.LISTEN_KEY, json.dumps(args.listen))
size = args.payload_size

count = 0
start = None
sleeping = False

def listener(sample):
    global count, sleeping
    if sleeping:
        count += 1

def main():
    global count, start, sleeping
    # initiate logging
    zenoh.init_logger()

    session = zenoh.open(conf)

    # By explicitly constructing the `Closure`, the `Queue` that's normally inserted between the callback and zenoh is removed.
    # Only do this if your callback runs faster than the minimum expected delay between two samples.
    sub = session.declare_subscriber("test/thr", zenoh.Closure((listener, lambda *args: None)), reliability=Reliability.RELIABLE())

    count = 0
    sleeping = True
    start = time.time()
    while True:
        time.sleep(1)
        if count > 0:
            sleeping = False
            end=time.time()
            print(f"{size},{(count / (end - start)):.3f}", flush=True)
            # reset counter and timer
            count = 0
            sleeping = True
            start = time.time()

    sub.undeclare()
    session.close()
    # while `sub.undeclare()` only returns once the unsubscription is done (no more callbacks will be queued from that instant), already queued callbacks may still be running in threads that Python can't see.
    time.sleep(0.1)

main()
