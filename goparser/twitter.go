package goparser

import (
	"encoding/json"
	"os"
)

var data []byte
var twitter *Twitter

func init() {
	data, _ = os.ReadFile("../testdata/twitter.json")
	json.Unmarshal(data, &twitter)
}

type Twitter struct {
	SearchMetadata struct {
		CompletedIn float64 `json:"completed_in"`
		Count       int64   `json:"count"`
		MaxID       int64   `json:"max_id"`
		MaxIDStr    string  `json:"max_id_str"`
		NextResults string  `json:"next_results"`
		Query       string  `json:"query"`
		RefreshURL  string  `json:"refresh_url"`
		SinceID     int64   `json:"since_id"`
		SinceIDStr  string  `json:"since_id_str"`
	} `json:"search_metadata"`
	Statuses []struct {
		Contributors interface{} `json:"contributors"`
		Coordinates  interface{} `json:"coordinates"`
		CreatedAt    string      `json:"created_at"`
		Entities     struct {
			Hashtags []struct {
				Indices []int64 `json:"indices"`
				Text    string  `json:"text"`
			} `json:"hashtags"`
			Urls         []interface{} `json:"urls"`
			UserMentions []interface{} `json:"user_mentions"`
		} `json:"entities"`
		Favorited            bool        `json:"favorited"`
		Geo                  interface{} `json:"geo"`
		ID                   int64       `json:"id"`
		IDStr                string      `json:"id_str"`
		InReplyToScreenName  interface{} `json:"in_reply_to_screen_name"`
		InReplyToStatusID    interface{} `json:"in_reply_to_status_id"`
		InReplyToStatusIDStr interface{} `json:"in_reply_to_status_id_str"`
		InReplyToUserID      interface{} `json:"in_reply_to_user_id"`
		InReplyToUserIDStr   interface{} `json:"in_reply_to_user_id_str"`
		Metadata             struct {
			IsoLanguageCode string `json:"iso_language_code"`
			ResultType      string `json:"result_type"`
		} `json:"metadata"`
		Place        interface{} `json:"place"`
		RetweetCount int64       `json:"retweet_count"`
		Retweeted    bool        `json:"retweeted"`
		Source       string      `json:"source"`
		Text         string      `json:"text"`
		Truncated    bool        `json:"truncated"`
		User         struct {
			ContributorsEnabled bool   `json:"contributors_enabled"`
			CreatedAt           string `json:"created_at"`
			DefaultProfile      bool   `json:"default_profile"`
			DefaultProfileImage bool   `json:"default_profile_image"`
			Description         string `json:"description"`
			Entities            struct {
				Description struct {
					Urls []interface{} `json:"urls"`
				} `json:"description"`
				URL struct {
					Urls []struct {
						ExpandedURL interface{} `json:"expanded_url"`
						Indices     []int64     `json:"indices"`
						URL         string      `json:"url"`
					} `json:"urls"`
				} `json:"url"`
			} `json:"entities"`
			FavouritesCount                int64       `json:"favourites_count"`
			FollowRequestSent              interface{} `json:"follow_request_sent"`
			FollowersCount                 int64       `json:"followers_count"`
			Following                      interface{} `json:"following"`
			FriendsCount                   int64       `json:"friends_count"`
			GeoEnabled                     bool        `json:"geo_enabled"`
			ID                             int64       `json:"id"`
			IDStr                          string      `json:"id_str"`
			IsTranslator                   bool        `json:"is_translator"`
			Lang                           string      `json:"lang"`
			ListedCount                    int64       `json:"listed_count"`
			Location                       string      `json:"location"`
			Name                           string      `json:"name"`
			Notifications                  interface{} `json:"notifications"`
			ProfileBackgroundColor         string      `json:"profile_background_color"`
			ProfileBackgroundImageURL      string      `json:"profile_background_image_url"`
			ProfileBackgroundImageURLHTTPS string      `json:"profile_background_image_url_https"`
			ProfileBackgroundTile          bool        `json:"profile_background_tile"`
			ProfileImageURL                string      `json:"profile_image_url"`
			ProfileImageURLHTTPS           string      `json:"profile_image_url_https"`
			ProfileLinkColor               string      `json:"profile_link_color"`
			ProfileSidebarBorderColor      string      `json:"profile_sidebar_border_color"`
			ProfileSidebarFillColor        string      `json:"profile_sidebar_fill_color"`
			ProfileTextColor               string      `json:"profile_text_color"`
			ProfileUseBackgroundImage      bool        `json:"profile_use_background_image"`
			Protected                      bool        `json:"protected"`
			ScreenName                     string      `json:"screen_name"`
			ShowAllInlineMedia             bool        `json:"show_all_inline_media"`
			StatusesCount                  int64       `json:"statuses_count"`
			TimeZone                       string      `json:"time_zone"`
			URL                            string      `json:"url"`
			UtcOffset                      int64       `json:"utc_offset"`
			Verified                       bool        `json:"verified"`
		} `json:"user"`
	} `json:"statuses"`
}
