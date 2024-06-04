import test from 'ava';

import { fibonacci } from '../index.js';

test('fibonacci sequence from native', (t) => {
  t.is(fibonacci(14), 377);
});
