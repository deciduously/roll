(ns roll.events
  (:require [re-frame.core :as re-frame]
            [roll.db :as db]
            [day8.re-frame.tracing :refer-macros [fn-traced defn-traced]]))

(re-frame/reg-event-db
 ::initialize-db
 (fn-traced [_ _]
   db/default-db))
