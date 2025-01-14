{
"title": "What Machine Learning can learn from Ruby on Rails",
"date": "January 1, 2022",
"author": {
"name": "Isabella Tromba",
"gravatar": "https://gravatar.com/avatar/b5c16153bae7a6fa6663d7f555906dd0?s=100"
}
}

## Convention over Configuration

I wrote my first end-to-end functioning web application using Ruby on Rails in [a class at MIT (6.170)](https://stellar.mit.edu/S/course/6/sp13/6.170/index.html) in 2013. There were things that Rails automatically handled for me that I didn’t even realize were hard to do. Running `rails new` just set up a completely functioning application. I never had to consider all of the components I would need to string together. Database migrations were handled and the documentation clearly described how to build every part of my application. In fact, I assumed that writing web applications should always be this easy because I had never tried to write one from scratch. I was the beginner benefiting from my own ignorance that DHH talks about in [The Rails Doctrine](https://rubyonrails.org/doctrine/)!

> But beyond the productivity gains for experts, conventions also lower the barriers of entry for beginners. There are so many conventions in Rails that a beginner doesn’t even need to know about, but can just benefit from in ignorance. It’s possible to create great applications without knowing why everything is the way it is.

> That’s not possible if your framework is merely a thick textbook and your new application a blank piece of paper. It takes immense effort to even figure out where and how to start. Half the battle of getting going is finding a thread to pull.

> \- DHH, The Rails Doctrine

Fast forward a couple of years, I ended up becoming a machine learning engineer at Slack. Unfortunately, getting machine learning into production felt a lot more like the framework as a thick textbook and my application as a blank piece of paper.

And to make things even worse, try googling “how to learn machine learning”. You will find countless articles explaining what you need to learn to get started. The lists start looking like the curriculum required to obtain a PhD in Statistics, Math, and Computer Science.

<!-- It’s as if the getting started with PostgreSQL tutorial went like this:

- Learn about [Hash joins](https://en.wikipedia.org/wiki/Hash_join), [Merge joins](https://en.wikipedia.org/wiki/Sort-merge_join), and [Nested loop joins](https://en.wikipedia.org/wiki/Nested_loop_join).
- Learn about [Two-phase locking](https://en.wikipedia.org/wiki/Two-phase_locking) and
  [B-Tree indexes](https://en.wikipedia.org/wiki/B-tree)

Instead, the postgres tutorial is more like this:

- `brew install postgres`
- Create a table
- Write a query
-->

And the problems don’t end once you have successfully trained a model. You still have to figure out how to get your model into production. The code you wrote in your jupyter notebook needs to be translated into code that can be deployed. An entirely new job called “Machine Learning Engineer” was created just to solve this problem. Machine learning engineers write code to transform raw data into features, choose a serialization format to save the model that was trained by the data scientists, set up servers to host those models, configure production monitoring to look for data quality and drift issues, set up automated data alerting and add A/B testing frameworks to test new models.

## Value integrated systems

In the Rails Doctrine, there is a section on “Value Integrated Systems”. DHH says that Rails is “A whole system that addresses an entire problem.”

> Rails can be used in many contexts, but its first love is the making of integrated systems: Majestic monoliths! A whole system that addresses an entire problem. This means Rails is concerned with everything from the front-end JavaScript needed to make live updates to how the database is migrated from one version to another in production.

> That’s a very broad scope, as we’ve discussed, but no broader than to be realistic to understand for a single person. Rails specifically seeks to equip generalist individuals to make these full systems. Its purpose is not to segregate specialists into small niches and then require whole teams of such in order to build anything of enduring value.
>
> \- DHH, The Rails Doctrine

Today, companies that want to get machine learning deployed in to production have to assemble a team of specialists including Data Scientists, Machine Learning Engineers, Backend engineers and Ops teams.

It would be great if we had something like Ruby on Rails for machine learning: a single system that provides the tools you need to go from data to a machine learning model deployed in production serving customers.

Today, application developers can use Ruby on Rails or Next.js to get a functioning web application working with litle configuration. They use PostgreSQL or MySQL to get a functioning database. And in the future, we hope application developers will have the tools to be able to train, deploy, and monitor machine learning models on their own.

## Introducing Tangram

Tangram is an all-in-one automated machine learning framework that makes it easy to add machine learning to your applications. Predictions happens directly in your existing applications so there are no network requests and there is no need to set up a separate service to serve your models.

- Run `tangram train` to train a model from a CSV file on the command line.
- Make predictions with bindings for [Ruby](https://rubygems.org/gems/tangram), [Python](https://pypi.org/project/tangram), [Golang](https://pkg.go.dev/github.com/tangramdotdev/tangram-go), [Elixir](https://hex.pm/packages/tangram), [Javascript](https://www.npmjs.com/package/@tangramdotdev/tangram), [PHP](https://packagist.org/packages/tangram/tangram), or [Rust](https://lib.rs/tangram).
- Run `tangram app` to start a web application where you can learn more about your models and monitor them in production.

Tangram is a new project and there is a lot of work ahead. We’d love to get your feedback. Check out the project on [GitHub](https://github.com/tangramdotdev/tangram), and let us know what you think!
