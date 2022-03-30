# assignment

![CI Status](https://github.com/Akagi201/flight-assignment/actions/workflows/test.yml/badge.svg?branch=master)

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

Define and document the format of the API endpoint in the README.

Use Rust and/or with any tools that you think will help you best accomplish the task at hand.

When you are done with the assignment and follow up via email with an estimate of how long you spent on the task and any interesting ideas you wish to share.

## Analysis

After studying the problem, I found that it is a typical Euler Graph problem.We can think of the location as point, and think of the flight as a arrow line. So the problem turns out to be how to find a path to walk though all the points.

Then the problem can be divided into a few steps:

Step 1: find the start point and the end point. When the two points connected with the arrow line, let's calcuate the start point *out* count add 1, and the end point *in* count add 1.

  > Scene 1: The graph is not loop, you can find that the final path start point is the one which `out - in = 1`, and the end point is the one which `in - out = 1`.
  > Scene 2: The graph is loop, the you can start from any point, and you will finally walk back to it.

Step 2: Then let's search from the start point with DFS and Backtrace. When there are many solutions, use min lexicographical order.

## Checklists

- [x] axum as web.
- [x] github CI test.
- [x] log elapsed time.
- [x] deploy online. curl example.
- [x] Dockerfile to deploy.
- [x] validate input.
- [x] full document to explain the algorithm.

## Solution API Server Online

I have deployed the solution to my own cloud server.

```sh
curl --location --request POST 'http://localhost:2333/solution' \
--header 'Content-Type: application/json' \
--data-raw '{
  "flights": [
        ["IND", "EWR"],
        ["SFO", "ATL"],
        ["GSO", "IND"],
        ["ATL", "GSO"]
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
