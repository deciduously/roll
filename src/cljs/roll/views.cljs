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
    (let [total (reduce + (map js/parseInt rolls))] ; maybe this should be a subscription?  get re-frame-y
      [:span.roll-total total])
    [:span.roll-vals (str ": " rolls)]]])

(defn outcomes
  "View a list of outcomes"
  [{:keys [time result]} outcomes]
  [:div (.toLocaleTimeString time js/Date)
   [:ul.outcomes
    (for [o (:outcomes result)]
                                        ;^{:key time} ; TODO how do I do this
      [outcome o])]])

(defn roll-hx
  "View full roll history"
  [hx]
  [:ul.hx
   (for [os (reverse hx)]
                                        ; Add time to each using cofx in evnet handler
     [:li [outcomes os]])])

(defn usage
  "Usage instructions"
  []
  [:div.usage
   [:p "Usage" [:br] "Enter commands in one of three formats:"]
   [:ul
    [:li "One or more rolls in XdX format: `1d6`, `1d4 2d20`, etc"]
    [:li "A multiplier followed by a roll: `6 2d6`"]
    [:li "An item to look up in the table, optionally preceeded by a multiplier: `ian`, `33 ian`"]]])

(defn main-panel []
  (let [result (re-frame/subscribe [::subs/results])
        error (re-frame/subscribe [::subs/error])]
    [:div [:h1 "ROLL"] [usage] "Roll history:  " [roll-hx @result] [:br] [command-input] [:br] "Last error:  " @error [:hr] [footer]]))
