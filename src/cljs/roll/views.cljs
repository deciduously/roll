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

(defn command-input
  "Command input"
  []
  [:div
   "Command: "
   [:input {:type "text"
            :id "field"
            :name "cmd"}] ;; do I need :name?
   [:input {:type "button"
            :value "Submit"
            :on-click #(re-frame/dispatch
                        [::events/submit-command (-> (.getElementById js/document "field") .-value)])}]])

; {:roll str :rolls Vec<u32>}

(defn outcome
  "View a single outcome"
  [{:keys [roll rolls]} outcome]
  [:li.roll
   [:span.roll-string roll]
   [:div.roll-result
    (let [total (reduce + (map js/parseInt rolls))]
      [:span.roll-total total])
    [:span.roll-vals (str ": " rolls)]]])

(defn outcomes
  "View a list of outcomes"
  [{:keys [outcomes]} outcomes]
  [:ul.outcomes
   (for [o outcomes]
                                        ;^{:key (:)} ; TODO how do I do this
     [outcome o])])

(defn main-panel []
  (let [result (re-frame/subscribe [::subs/result])]
    [:div "Last result:  " [outcomes @result] [:br] [command-input] [:hr] [footer]]))
