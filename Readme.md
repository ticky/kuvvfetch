# kuvvfetch

Fetch wallpapers from your Kuvva favourites, now that the apps are dead

## What?

[Kuvva](https://kuvva.com) is a now-on-indefinite-hiatus site which used to
provide a large library of wallpapers commissioned from artists. The collection
updated frequently. iOS and Mac apps which would sync your set of favourite
wallpapers, and, in the case of the Mac OS app, rotate them on a schedule were
available to download or purchase.

The apps are no longer available on the App Store, and despite once owning the
iOS version, I can't access it in my history either. It looks like they're very
dead!

Even still, it used to be possible to download the wallpapers from the website,
in the resolution of your current monitor, via a button on each wallpaper's
page. This seems to have disappeared too!

So this provides a small utility to fetch your favourites. It doesn't know which
resolutions are valid, but Kuvva have some internal set of resolutions they
appear to store the wallpapers at. The defaults are good for forwards
compatibility, and seem to be the largest size that exists, but not all
submissions are available in that resolution, so YMMV.

## How?

Go to your Kuvva favourites, enable and copy your public sharing url.

Clone this repository, install Rust, and run `cargo run -- <share-url>`

Wallpapers will end up in your curreny directory. You can also specify to put
them elsewhere by adding that as a second parameter.