# TSAK is a Telemetry Swiss Army Knife.

and a programmatic tool combined with Domain Specific Language geared towards common tasks related to obtaining, parsing, processing, computing, generating, analyzing and delivering observability metrics. This tool has been developed as computation and integration aide for a New Relic observability platform. What are the use cases for a TSAK ?

* Feel that New Relic provided Flex mechanism is inadequate for you custom integrations ?
* Need a testing tool for programmatic generation of the observability data ?
* Want to create a custom Prometheus scrubber that uses native New Relic data API ?
* Do you want to post-process data that's already been submitted to a New Relic and derive aggregation and post-compute Events or Metrics ?
* Do you want an easy interface to a Neural Network for data analysis ?
* Do you want to use rich library of statistical functions with your live telemetry data ?

TSAK provides all of this and more.

## What is TSAK ?

The TSAK is a programmatic instrument designed to obtain and  transform data of various format from variety of sources and convert this data to or generate a set of telemetry data that is submitted to New Relic platform. Rather than very rigid and curated instrument the old TSAK was, the new TSAK is just a programming language that is close to it’s “look and feel” to JavaScript or any other languages of that type, enhanced with specialized primitives that helping to process data and generate telemetry.

## Show me how you send a Metric

```lang=rust
let event = Event();
event["instanceId"] = INSTANCE;
event["answer"] = 42;
event.send(NR_EVENT, NR_ACCOUNT, NR_INSERT_KEY);
```

First, you've created an instance of Event(), then you set some event attributes and with event.send() you are sending prepared event to New Relic

## SHow me something cool

How about having Neural Network suitable for telemetry data analysis availabled as a part of standard library ?

```lang=rust
let nn = NeuralNetwork(2,3,1);
nn.add([0.0, 0.0], [0.0]);
nn.add([0.0, 1.0], [1.0]);
nn.add([1.0, 0.0], [1.0]);
nn.add([1.0, 1.0], [0.0]);
nn.train();
let res1 = nn.forward([0.0, 0.0]);
let res2 = nn.forward([1.0, 0.0]);
print(`0 XOR 0 is likely ${res1}`);
print(`1 XOR 0 is likely ${res2}`);
```

First we creating neural network with 2 input layers, one output and three hidden. Then we running some training, but don't expect a good outcome with just a four training samples. Then we are trying to pass data forward to neural net.
