import sys
from pathlib import Path
import pandas as pd
import numpy as np


assert len(sys.argv) == 2
data_dir = Path(sys.argv[1])

def percentile(p):
    assert 0 <= p <= 100
    return lambda vec: np.percentile(vec, p)


MAP = {
    'p01': percentile(1),
    'p05': percentile(5),
    'p50': percentile(50),
    'p95': percentile(95),
    'p99': percentile(99),
}


def load(file):
    cols = str(file).split('/')
    data = pd.read_csv(file)
    data.columns = ['payload', 'Gbps']
    data['binding'] = cols[-2].split('-')[-1]
    data['branch'] = cols[-3]
    data['Gbps'] *= data['payload'] * 8 / 1e9
    return data

pd.set_option("display.precision", 2)

data = pd.concat(map(load, data_dir.glob('**/throughput.log')))
data = data.groupby(['payload', 'binding', 'branch']).agg(list(MAP.values()))
data.columns = MAP.keys()
data = data.unstack()

print(data)
