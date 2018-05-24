(ns roll.views
  (:require [re-frame.core :as re-frame]
            [roll.events :as events]
            [roll.subs :as subs]
            ))

(defn http-link
  ([url] [:a {:href url} url])
  ([url title] [:a {:href url} title]))

(defn footer
  "Page Footer"
  []
  [:footer
   (str \u00A9 " 2018 deciduously - ")
   (http-link "https://github.com/deciduously/roll")])

(defn command-input []
  [:div
   "Command: "
   [:input {:type "text"
            :id "field"
            :name "cmd"}]
   [:input {:type "button" :value "Submit" :on-click #(re-frame/dispatch [::events/submit-command (-> (.getElementById js/document "field") .-value)])}]])

(defn main-panel []
  (let [result (re-frame/subscribe [::subs/result])]
    [:div "Last result:  " @result [:br] [command-input] [footer]]))