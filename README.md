# loadtest-blog

A simple load test that primarily loads a single highly popular blog page from our website.

![angry-goose-white-background-42416907](https://user-images.githubusercontent.com/402892/115951679-4a3ce100-a4e2-11eb-8ebc-3ca8a7da1491.jpg)

See src/main.rs:
 - 95% of the users load a single high-interest blog;
 - 4% of the users load the front page (i.e. to learn more about Tag1);
 - 1% of the users load the blog listing page (i.e. to look for more content).

Run this load test with a number of users divisible by 100 if you want the exact distribution as described above.

The following flags are recommended when running this load test:

```bash
    cargo run --release -- --log-file goose.log -g -H https://tag1consulting.com/ \
         --debug-file debug.log --requests-file requests.log \
         --status-codes --hatch-rate 5 -u100 -v
```

 * Adjust `-u100` to the desired number of users;
 * Adjust `--hatch-rate 5` to your desired ramp up speed;
 * Collect and/or review the verbose output from the load test in `goose.log`;
 * Collect and/or review all requests made in `requests.log`. (For example `grep -v 200 requests.log` to show all requests that didn't return a 200.);
 * Collect and/or review complete details around any errors in the `debug.log`;
 * If you want to collect the metrics summary in a file, add `> metrics.log` to the end of the above command;
 * Review https://www.tag1consulting.com/blog/real-life-goose-load-testing for more ideas on how to leverage these flags.
