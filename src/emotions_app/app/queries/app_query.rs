
    pub struct PageQuery{
        current_page:i32,
        item_per_page:i32
    }

    pub struct SearchEmotionsQuery{
        page_query:PageQuery,
        keywords:String
    }



