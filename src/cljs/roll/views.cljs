(ns roll.views
  (:require [re-frame.core :as re-frame]
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

(defn main-panel []
  (let [name (re-frame/subscribe [::subs/name])]
    [:div "Hello from " @name [footer]]))
