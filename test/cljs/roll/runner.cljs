(ns roll.runner
    (:require [doo.runner :refer-macros [doo-tests]]
              [roll.core-test]))

(doo-tests 'roll.core-test)
