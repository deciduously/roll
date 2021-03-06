(ns roll.core
  (:require [reagent.core :as reagent]
            [re-frame.core :as re-frame]
            [roll.events :as events]
            [roll.views :as views]
            [roll.config :as config]))


(defn dev-setup []
  (when config/debug?
    (enable-console-print!)
    (println "dev mode")))

(defn mount-root []
  (re-frame/clear-subscription-cache!)
  (reagent/render [views/main-panel]
                  (.getElementById js/document "app")))

(defn ^:export init []
  (re-frame/dispatch-sync [::events/initialize-db])
  (re-frame/dispatch [::events/get-items])
  (dev-setup)
  (mount-root))
