use tracing::{info, span, trace, Level};

pub(crate) fn tracing_main() {
    tracing_subscriber::fmt()
        // enable everything
        .with_max_level(tracing::Level::TRACE)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        // sets this to be the default, global collector for this application.
        .init();

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    tracing::info!(number_of_yaks, "preparing to shave yaks");

    let number_shaved = |yaks: usize| -> usize {
        // Constructs a new span named "shaving_yaks" at the INFO level,
        // and a field whose key is "yaks". This is equivalent to writing:
        //
        // let span = span!(Level::INFO, "shaving_yaks", yaks = yaks);
        //
        // local variables (`yaks`) can be used as field values
        // without an assignment, similar to struct initializers.
        let span = span!(Level::INFO, "shaving_yaks", yaks);
        let _enter = span.enter();

        info!("shaving yaks");

        let mut yaks_shaved = 0;
        for yak in 1..=yaks {
            yaks_shaved += 1;
            trace!(yaks_shaved);
        }

        yaks_shaved
    }(number_of_yaks);
    tracing::info!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        "yak shaving completed."
    );
}
