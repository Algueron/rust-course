/*
    Define a Tier enum with three variants: Gold, Silver,
    Platinum. Derive a Debug implementation for the Tier enum.
*/
#[derive(Debug)]
enum Tier {
    Gold,
    #[allow(dead_code)]
    Silver,
    #[allow(dead_code)]
    Platinum
}

/*
    Define a Subscription enum with three variants: Free,
    Basic, and Premium. Derive a Debug implementation for the
    Subscription enum.
*/
#[derive(Debug)]
enum Subscription {
    /*
        The Free variant should have no associated data.
    */
    Free,

    /*
        The Basic variant should be a tuple variant with two pieces
        of data. The first one should be a f64 (the price per month)
        and the second one should be a u32 (the number of months).
    */
    Basic(f64, u32),

    /*
        The Premium variant should be a struct variant with a 'tier'
        field. The tier field should be a Tier enum value.
    */
    Premium { tier: Tier }
}

/*
    Define a 'summarize' method on the Subscription enum.
*/
fn summarize(subscription: &Subscription) {
    
    match subscription {
        /*
            If the Subscription is Free, output the text "You have
            limited access to the site".
        */
        Subscription::Free => {
            println!("You have limited access to the site");
        },

        /*
            If the Subscription is Basic, output the text "You have
            limited access to the site's premium features for {price}
            for {months} months", where {price} amd {months} come from
            the tuple variant's associated data.
        */
        Subscription::Basic(price, months ) => {
            println!("You have limited access to the site's premium features for {price} for {months} months");
        },

        /*
            If the Subscription is Premium, output the text "You have
            full access to the site's premium features. Your tier is
            {tier:?}"", where {tier} comes from the struct variant's
            associated 'tier' field.
        */
        Subscription::Premium { tier } => {
            println!("You have full access to the site's premium features. Your tier is {tier:?}");
        }
    }

}

fn main() {
    /*
        In the `main` function, create 3 instances of Subscription,
        each one with a different variant. Invoke the `summarize`
        method on each instance.
    */
    let free_subscription = Subscription::Free;
    summarize(&free_subscription);

    let basic_subscription = Subscription::Basic(45.99, 18);
    summarize(&basic_subscription);

    let premium_subscription = Subscription::Premium { tier: Tier::Gold };
    summarize(&premium_subscription);
}
