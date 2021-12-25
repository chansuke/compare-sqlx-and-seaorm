# active_timer(WIP)

`activities`
|name|type|NULL|KEY|NOTE|
|---|---|---|---|---|
|id|INTEGER|NO|PRIMARY||
|name|TEXT|NO||活動の名前。ex: TaPL 11章~12章|
|user_id|INTEGER|NO|FOREIGN| |
|category_id|INTEGER|NO|FOREIGN|カテゴリー|
|created_at|DATETIME|NO|   | |
|updated_at|DATETIME|YES|   |   |
|deleted_at|DATETIME|YES|   |   |

`durations`
|name|type|NULL|KEY|NOTE|
|---|---|---|---|---|
|id|INTEGER|NO|PRIMARY||
|activity_id|INTEGER|NO|FOREIGN|活動|
|start_date|DATETIME|YES|   |開始時間|
|end_date|DATETIME|YES|   |終了時間|
|created_at|DATETIME|NO|   | |
|updated_at|DATETIME|YES|   |   |
|deleted_at|DATETIME|YES|   |   |

`categories`
|name|type|NULL|KEY|NOTE|
|---|---|---|---|---|
|id|INTEGER|NO|PRIMARY||
|name|TEXT|NO||カテゴリーの名前。ex: 勉強|
|sub_category_id|INTEGER|YES|FOREIGN||
|created_at|DATETIME|NO| ||
|updated_at|DATETIME|YES| | |
|deleted_at|DATETIME|YES| | |

`sub_categories`
|name|type|NULL|KEY|NOTE|
|---|---|---|---|---|
|id|INTEGER|NO|PRIMARY||
|name|TEXT|NO||サブカテゴリーの名前。ex: 数学|
|created_at|DATETIME|NO| | |
|updated_at|DATETIME|YES| | |
|deleted_at|DATETIME|YES| | |

`users`
|name|type|NULL|KEY|NOTE|
|---|---|---|---|---|
|id|INTEGER|NO|PRIMARY| |
|name|TEXT|NO| | |
|email|TEXT|YES| | |
|image|TEXT|YES| |アイコン|
|description|TEXT|YES| |紹介文|
|created_at|DATETIME|NO| | |
|updated_at|DATETIME|YES| | |
|deleted_at|DATETIME|YES| | |
