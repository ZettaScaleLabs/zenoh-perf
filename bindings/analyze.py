import numpy as np


for bind in ['zenoh-c', 'zenoh-cpp']:
    for branch in ['main', 'tokio']:
        name = f'{bind}/{branch}'
        print()
        print(name)
        data = np.loadtxt(f'./_results/{name}.log', delimiter=',')[:, 1]
        data.sort()

        print('p01: {}, p05: {}, p50: {}, p95: {}, p99: {}'.format(
            data[int(len(data) * 0.01)],
            data[int(len(data) * 0.05)],
            data[int(len(data) * 0.50)],
            data[int(len(data) * 0.95)],
            data[int(len(data) * 0.99)],
        ))
