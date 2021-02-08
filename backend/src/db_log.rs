use crate::error::Error;
use crate::Result;
use crossbeam_channel::{unbounded, Sender};
use std::thread;

#[derive(Debug)]
struct Field {
    field_name: String,
    value: String,
}

#[derive(Debug)]
enum Operation {
    Update { id: i32, values: Vec<Field> },
    Insert { values: Vec<Field> },
}

#[derive(Debug)]
struct Log {
    table_name: String,
    operation: Operation,
}

pub struct Logger {
    sender: Sender<Log>,
}

impl Logger {
    pub fn new() -> Logger {
        // Create a channel of unbounded capacity.
        let (sender, receiver) = unbounded();
        thread::spawn(move || {
            let mut cont = true;
            while cont {
                match receiver.recv() {
                    Ok(log) => println!("{:?}", log),
                    _ => cont = false,
                };
            }
        });
        Logger { sender }
    }

    pub fn update1(
        &self,
        table_name: String,
        id: i32,
        field_name: String,
        value: String,
    ) -> Result<()> {
        self.sender
            .send(Log {
                table_name: table_name,
                operation: Operation::Update {
                    id: id,
                    values: vec![Field {
                        field_name: field_name,
                        value: value,
                    }],
                },
            })
            .map_err(|_| Error::LoggerError)?;
        Ok(())
    }

    pub fn update2(
        &self,
        table_name: String,
        id: i32,
        field_name1: String,
        value1: String,
        field_name2: String,
        value2: String,
    ) -> Result<()> {
        self.sender
            .send(Log {
                table_name: table_name,
                operation: Operation::Update {
                    id: id,
                    values: vec![
                        Field {
                            field_name: field_name1,
                            value: value1,
                        },
                        Field {
                            field_name: field_name2,
                            value: value2,
                        },
                    ],
                },
            })
            .map_err(|_| Error::LoggerError)?;
        Ok(())
    }

    pub fn update5(
        &self,
        table_name: String,
        id: i32,
        field_name1: String,
        value1: String,
        field_name2: String,
        value2: String,
        field_name3: String,
        value3: String,
        field_name4: String,
        value4: String,
        field_name5: String,
        value5: String,
    ) -> Result<()> {
        self.sender
            .send(Log {
                table_name: table_name,
                operation: Operation::Update {
                    id: id,
                    values: vec![
                        Field {
                            field_name: field_name1,
                            value: value1,
                        },
                        Field {
                            field_name: field_name2,
                            value: value2,
                        },
                        Field {
                            field_name: field_name3,
                            value: value3,
                        },
                        Field {
                            field_name: field_name4,
                            value: value4,
                        },
                        Field {
                            field_name: field_name5,
                            value: value5,
                        },
                    ],
                },
            })
            .map_err(|_| Error::LoggerError)?;
        Ok(())
    }

    pub fn insert1(&self, table_name: String, field_name1: String, value1: String) -> Result<()> {
        self.sender
            .send(Log {
                table_name: table_name,
                operation: Operation::Insert {
                    values: vec![Field {
                        field_name: field_name1,
                        value: value1,
                    }],
                },
            })
            .map_err(|_| Error::LoggerError)?;
        Ok(())
    }

    pub fn insert2(
        &self,
        table_name: String,
        field_name1: String,
        value1: String,
        field_name2: String,
        value2: String,
    ) -> Result<()> {
        self.sender
            .send(Log {
                table_name: table_name,
                operation: Operation::Insert {
                    values: vec![
                        Field {
                            field_name: field_name1,
                            value: value1,
                        },
                        Field {
                            field_name: field_name2,
                            value: value2,
                        },
                    ],
                },
            })
            .map_err(|_| Error::LoggerError)?;
        Ok(())
    }

    pub fn insert3(
        &self,
        table_name: String,
        field_name1: String,
        value1: String,
        field_name2: String,
        value2: String,
        field_name3: String,
        value3: String,
    ) -> Result<()> {
        self.sender
            .send(Log {
                table_name: table_name,
                operation: Operation::Insert {
                    values: vec![
                        Field {
                            field_name: field_name1,
                            value: value1,
                        },
                        Field {
                            field_name: field_name2,
                            value: value2,
                        },
                        Field {
                            field_name: field_name3,
                            value: value3,
                        },
                    ],
                },
            })
            .map_err(|_| Error::LoggerError)?;
        Ok(())
    }

    pub fn insert4(
        &self,
        table_name: String,
        field_name1: String,
        value1: String,
        field_name2: String,
        value2: String,
        field_name3: String,
        value3: String,
        field_name4: String,
        value4: String,
    ) -> Result<()> {
        self.sender
            .send(Log {
                table_name: table_name,
                operation: Operation::Insert {
                    values: vec![
                        Field {
                            field_name: field_name1,
                            value: value1,
                        },
                        Field {
                            field_name: field_name2,
                            value: value2,
                        },
                        Field {
                            field_name: field_name3,
                            value: value3,
                        },
                        Field {
                            field_name: field_name4,
                            value: value4,
                        },
                    ],
                },
            })
            .map_err(|_| Error::LoggerError)?;
        Ok(())
    }

    pub fn insert5(
        &self,
        table_name: String,
        field_name1: String,
        value1: String,
        field_name2: String,
        value2: String,
        field_name3: String,
        value3: String,
        field_name4: String,
        value4: String,
        field_name5: String,
        value5: String,
    ) -> Result<()> {
        self.sender
            .send(Log {
                table_name: table_name,
                operation: Operation::Insert {
                    values: vec![
                        Field {
                            field_name: field_name1,
                            value: value1,
                        },
                        Field {
                            field_name: field_name2,
                            value: value2,
                        },
                        Field {
                            field_name: field_name3,
                            value: value3,
                        },
                        Field {
                            field_name: field_name4,
                            value: value4,
                        },
                        Field {
                            field_name: field_name5,
                            value: value5,
                        },
                    ],
                },
            })
            .map_err(|_| Error::LoggerError)?;
        Ok(())
    }
}
