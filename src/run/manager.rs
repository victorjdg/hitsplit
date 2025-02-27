use crate::HitSplit;

pub fn prev_split(app: &mut HitSplit) {
    if app.selected_split > 0 {
        app.selected_split -= 1;
    }
}

pub fn next_split(app: &mut HitSplit) {
    if let Some(category) = app.loaded_category.as_mut() {
        if category.splits.len() > app.selected_split + 1 {
            app.selected_split += 1;
        } else if app.config.next_split_as_reset {
            app.selected_split = 0;
            let pbs = category.splits.iter().map(|split| split.pb).sum::<u16>();
            let hits = category.splits.iter().map(|split| split.hits).sum::<u16>();
            category.splits.iter_mut().for_each(|split| {
                if pbs > hits {
                    split.pb = split.hits;
                }
                split.hits = 0;
            });
        }
    }
}

pub fn sub_hit(app: &mut HitSplit) {
    if let Some(category) = app.loaded_category.as_mut() {
        let split = category.splits.get_mut(app.selected_split).unwrap();
        if split.hits > 0 {
            split.hits -= 1;
        }
    }
}

pub fn add_hit(app: &mut HitSplit) {
    if let Some(category) = app.loaded_category.as_mut() {
        category.splits.get_mut(app.selected_split).unwrap().hits += 1;
    }
}

pub fn reset(app: &mut HitSplit) {
    if let Some(category) = app.loaded_category.as_mut() {
        category.splits.iter_mut().for_each(|split| {
            split.hits = 0;
        });
        app.selected_split = 0;
    }
}

pub fn set_pb(app: &mut HitSplit) {
    if let Some(category) = app.loaded_category.as_mut() {
        category.splits.iter_mut().for_each(|split| {
            split.pb = split.hits;
        });
    }
}
