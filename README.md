# volume-assignment

![CI Status](https://github.com/Akagi201/volume-assignment/actions/workflows/test.yml/badge.svg?branch=master)

## Problem

Senior Software Engineer Take-Home Programming Assignment for Rust

Story: There are over 100,000 flights a day, with millions of people and cargo being transferred around the world. With so many people, and different carrier/agency groups it can be hard to track where a person might be. In order to determine the flight path of a person, we must sort through all of their flight records.

Goal: To create a microservice API that can help us understand and track how a particular person’s flight path may be queried. The API should accept a request that includes a list of flights, which are defined by a source and destination airport code. These flights may not be listed in order and will need to be sorted to find the total flight paths starting and ending airports.

Examples:

```sh
[['SFO', 'EWR']]                                                 => ['SFO', 'EWR']
[['ATL', 'EWR'], ['SFO', 'ATL']]                                 => ['SFO', 'EWR']
[['IND', 'EWR'], ['SFO', 'ATL'], ['GSO', 'IND'], ['ATL', 'GSO']] => ['SFO', 'EWR']
```

Specifications:

Create a private GitHub repo and add <https://github.com/taariq> and <https://github.com/Vizualni> as collaborators to the project.

Define and document the format of the API endpoint in the README.

Use Rust and/or with any tools that you think will help you best accomplish the task at hand.

When you are done with the assignment and follow up via email with an estimate of how long you spent on the task and any interesting ideas you wish to share.

## Checklists

- [x] axum as web.
- [x] github ci. test.
- [x] log elapsed time.
- [ ] validate input.
- [ ] custom error types.
- [ ] multi answers.
- [ ] no answers.
- [ ] deploy online. curl example.
- [ ] Dockerfile to deploy.
- [ ] full document to explain the algorithm.

## Solution API Server Online

I have deployed the solution to my own cloud server.

```sh
curl --location --request POST 'http://localhost:8080/solution' \
--header 'Content-Type: application/json' \
--data-raw '{
  "flights": [
        ["MUC", "LHR"],
        ["JFK", "MUC"],
        ["SFO", "SJC"],
        ["LHR", "SFO"]
    ]
}'
```

## APIs

`Get /health`

will always return `OK`

`Post /solution`

### Request schema

JSON Format

```json
{
  "flights": [
    ["SFO", "EWR"],
    ["ATL", "EWR"],
    ["IND", "EWR"],
    ["SFO", "ATL"],
    ["GSO", "IND"],
    ["ATL", "GSO"]
  ]
}
```

### Response schema

JSON Format

```json
{
  "code": 0,
  "message": "OK",
  "answer": ["SFO", "EWR"],
}
```
