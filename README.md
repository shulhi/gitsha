# GitSHA

![GitSHA](https://i.imgur.com/Q4AgJpW.png)

A simple tool to retrieve latest commit's SHA for a given repo

### Usage

#### Configure
```
$ gitsha configure <GITHUB-API-TOKEN>
```

#### Retrieve commit's SHA
```
$ gitsha get shulhi/gitsha
$ gitsha get -b develop shulhi/gitsha # specific branch
```

### Why?

Because I deal with [Stack](https://docs.haskellstack.org/en/stable/README/) a lot, and due to microservices architecture, I often found the need to update multiple entries in `stack.yaml` to point to certain commit. It bugs me that I need to open up each repo's page and copy the latest commit. This solves part of the issue.
