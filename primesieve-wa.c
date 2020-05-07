#include "primesieve/iterator.h"

#ifdef __cplusplus
extern "C" {
#endif

uint64_t primesieve_next_prime_wa(primesieve_iterator *it) {
  return primesieve_next_prime(it);
}

uint64_t primesieve_prev_prime_wa(primesieve_iterator *it) {
  return primesieve_prev_prime(it);
}

#ifdef __cplusplus
} /* extern "C" */
#endif
