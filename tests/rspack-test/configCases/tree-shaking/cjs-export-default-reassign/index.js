import def from './dep.cjs';
import { foo } from './named.cjs';

it('keeps the reassigned CJS default export value (innerGraph on, production)', () => {
  // #14589: the chained `var _default = (exports.default = value)` write must
  // NOT be folded into the unused `_default` local and dropped.
  expect(def).toBe(42);
});

it('keeps a reassigned CJS named export value', () => {
  expect(foo).toBe(7);
});
