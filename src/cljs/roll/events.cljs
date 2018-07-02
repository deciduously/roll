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

(defonce last-temp-id (atom 0))

(re-frame/reg-cofx
 :temp-id
 (fn-traced [cofx _]
            (assoc cofx :temp-id (swap! last-temp-id inc))))

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
 ::get-items
 (fn-traced [_ [_ _]]
            {:http-xhrio {:method :get
                          :uri "http://localhost:8080/items"
                          :timeout 8000
                          :response-format (ajax/json-response-format {:keywords? true})
                          :on-success [::save-items]
                          :on-failure [::bad-http-result]}}))

;; try `clj-js` fn - (.stringify js/JSON (clj->js {:key "value"}))
;; nvm it didnt work, I think Im sending an empty body somehow

(re-frame/reg-event-fx
 ::add-item
 (fn-traced [_ [_ data]]
            {:http-xhrio {:method :post
                          :uri "http://localhost:8080/item"
                          :timeout 8000
                          :body (.stringify js/JSON
                                            {:name (-> (.getElementById js/document "item-name") .-value)
                                             :damage (-> (.getElementById js/document "item-damage") .-value)})
                          :format (ajax/json-request-format)
                          :response-format (ajax/json-response-format)
                          :on-success [::get-items]
                          :on-failure [::bad-http-result]}}))

;; TODO add a unique ID here
(re-frame/reg-event-fx
 ::save-roll
 [(re-frame/inject-cofx :now) (re-frame/inject-cofx :temp-id)]
 (fn-traced [{:keys [db temp-id now]} [_ result]]
            {:db (update db :roll-hx conj {:id temp-id :time now :result result})}))

(re-frame/reg-event-db
 ::save-items
 (fn-traced [db [_ result]]
            (assoc db :items result)))

(re-frame/reg-event-db
 ::bad-http-result
 (fn-traced [db [_ result]]
   (assoc db :api-error result)))
