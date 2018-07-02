(ns roll.subs
  (:require [re-frame.core :as re-frame]))

(re-frame/reg-sub
 ::name
 (fn [db]
   (:name db)))

(re-frame/reg-sub
 ::results
 (fn [db]
   (:roll-hx db)))

(re-frame/reg-sub
 ::error
 (fn [db]
   (:api-error db)))

(re-frame/reg-sub
 ::items
 (fn [db]
   (:items (:items db))))
