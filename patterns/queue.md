# Message queuing
**Message queuing** (**MQ**) — **design pattern** for **async interaction** between distributed components.

<br>

**Message queuing** = **Messaging** + **Queuing**

<br>

## Roles in MQ
**MQ broker** (aka Enterprise Service Bus, ESB) - some soft that implements MQ protocol.
**MQ clients**:
- **Publisher** (aka **Producer**) - who sends messages to *MQ broker*.
- **Consumer** (aka **Subscriber**) - who receives messages from *MQ broker*.

<br>

## Messaging
Messaging includes following steps:
1. **Publishing** (the act of sending mesage to *MQ broker*).
2. **Putting to queue**.
3. **Delivering** (aka **Propogation**) (*with* or *without* **ack**).

<br>

## Delivery schemas
- **1-to-1**  (**unicast**)
- **1-to-many** (aka **fan-out**, **broadcast**)

<br>

## Delivery guarantees
- **At-most-once** (it is impossible to retrieve the message again after it was sent to consumers that was seen at moment of **dequeuing**)
- **At-least-once** (it ensures the message will be processed, but, the message may be processed many times)
- **Exactly-once** (it ensures the message will be handled once)

<br>

## Persistance
**Persistence** means whether the message will disappear after it is sent to the system.

<br>

## MQ protocols
There are several protocols that implement MQ, the most popular are
- **AMQP** (Advanced Message Queuing Protocol).
- **MQTT** (MQ Telemetry Transport)

<br>

# AMQP
There are several implemetation of AMQP:
- **RabbitMQ**.
- Apache ActiveMQ.
- Apache Qpid.

<br>

There are several versions of AMQP protocol. **RabbitMQ** uses **AMQP** of *version* **0-9-1**.

<br>

## Basics terms of AMQP 0-9-1
- **Message** - the unit of passing data in AMQP. Every message has **attributes** (aka **message metadata**) and **payload**.
- **Exchange** - component that **receives** and **routes** messages.
- **Queue** - the **storage** for messages. Messages are stored in queue until they are delivered.
- **Broker** - **Exchange** + **Queue**.
- **Message acknowledgment** - if **acknowledgment** is enabled, the **broker** will only remove messages from the queue when it **receives a notification** from the consumer.

<br>

## Messaging steps
1. **Publishing**. **Publisher** sends **message** with **routing key** to appropriate **exchange**.
2. **Exchange** **receives** messages and **distributes** them between **queues** depending on **rules** and **exchange type**. 
3. Every rule binds **exchange**, **queue** and **routing key**. Example of rule: ```BIND <queue> TO <exchange> WITH <routing_key>.```.
4. Delivering to appropriate consumers (*with* or *without* **ack**).
