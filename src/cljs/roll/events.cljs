(ns roll.events
  (:require [re-frame.core :as re-frame]
            [roll.db :as db]
            [day8.re-frame.tracing :refer-macros [fn-traced defn-traced]]
            [day8.re-frame.http-fx]
            [ajax.core :as ajax]))

(re-frame/reg-event-db
 ::initialize-db
 (fn-traced [_ _]
   db/default-db))

(re-frame/reg-cofx
 :now
 (fn-traced [cofx _data]
            (assoc cofx :now (js/Date.))))

(re-frame/reg-event-fx
 ::submit-command
 (fn-traced [_ [_ cmd]]
   {:http-xhrio {:method :get
                 :uri (str "http://localhost:8080/roll/" (clojure.string/replace cmd #" " "/"))
                 :timeout 8000
                 :response-format (ajax/json-response-format {:keywords? true})
                 :on-success [::save-roll]
                 :on-failure [::bad-http-result]}}))

(re-frame/reg-event-fx
 ::save-roll
 [(re-frame/inject-cofx :now)]
 (fn-traced [cofx [_ result]]
   {:db (update (:db cofx) :roll-hx conj {:time (:now cofx) :result result})}))

(re-frame/reg-event-db
 ::bad-http-result
 (fn-traced [db [_ result]]
   (assoc db :api-error result)))
