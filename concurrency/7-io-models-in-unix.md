# I/O models in Unix
## Blocking (synchronous) I/O model
**Block** if request **cannot** be completed **immediately**.<br>
By default, **all file descriptors** on Unix systems start out in **blocking mode**.<br>
That means that I/O system calls like `read`, `write`, or `connect` will be **blocked**.

<br>

## Non-blocking I/O model
**Don't block** if request **cannot** be completed **immediately**, returns error `EWOULDBLOCK` instead.

<br>

## I/O multiplexing model
I/O multiplexing system calls:
- `select()`
- `epoll()`
- `kqueue()`

I/O multiplexing system calls can be **blocking** and **non-blocking**.

<br>

## Signal-driven I/O model
Tell kernel to notify application with `SIGIO` signal when the descriptor is ready.

<br>

## Asynchronous I/O model
Telling the kernel to start the operation and to notify application when the entire operation is complete.

<br>

Asynchronous I/O systemcall family:
- `aio_write()`
- `aio_read()`

<br>

# Example: enable non-blocking mode in linux
To put `fd` into **non-blocking mode** it is needed add `O_NONBLOCK` to the set of `fcntl` **flags**:
```c
/* set O_NONBLOCK on fd */
int flags = fcntl(fd, F_GETFL, 0);
fcntl(fd, F_SETFL, flags | O_NONBLOCK);
```

<br>

# Epoll
`epoll()` supports 2 modes:
- **level triggered**;
- **edge triggered**.

<br>

### Level triggered
- `fd` was added to `epoll` with `EPOLLIN` flag;
- `epoll_wait()` is blocked until new data will be written to `fd` buffer;
- write to file 19 bytes;
- `epoll_wait()` is unblocked with `EPOLLIN`;
- do nothing with data;
- `epoll_wait()` is unblocked with `EPOLLIN` again;

<br>

### Edge triggered
- `fd` was added to `epoll` with `EPOLLIN` flag;
- `epoll_wait()` is blocked until new data will be writeen to `fd` buffer;
- write to file 19 bytes;
- `epoll_wait()` is unblocked with `EPOLLIN`;
- do nothing with data;
- `epoll_wait()` is blocked until new data will be written to `fd` buffer;
