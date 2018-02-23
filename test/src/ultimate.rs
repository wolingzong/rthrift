// Autogenerated by Thrift Compiler (0.11.0)
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments, type_complexity))]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate ordered_float;
extern crate thrift;
extern crate try_from;

use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::From;
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use try_from::TryFrom;

use thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TInputProtocol, TOutputProtocol, TSetIdentifier, TStructIdentifier, TType};
use thrift::protocol::field_id;
use thrift::protocol::verify_expected_message_type;
use thrift::protocol::verify_expected_sequence_number;
use thrift::protocol::verify_expected_service_call;
use thrift::protocol::verify_required_field_exists;
use thrift::server::TProcessor;

use base_two;
use midlayer;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Drink {
  WATER = 0,
  WHISKEY = 1,
  WINE = 2,
}

impl Drink {
  pub fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    o_prot.write_i32(*self as i32)
  }
  pub fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<Drink> {
    let enum_value = i_prot.read_i32()?;
    Drink::try_from(enum_value)  }
}

impl TryFrom<i32> for Drink {
  type Err = thrift::Error;  fn try_from(i: i32) -> Result<Self, Self::Err> {
    match i {
      0 => Ok(Drink::WATER),
      1 => Ok(Drink::WHISKEY),
      2 => Ok(Drink::WINE),
      _ => {
        Err(
          thrift::Error::Protocol(
            ProtocolError::new(
              ProtocolErrorKind::InvalidData,
              format!("cannot convert enum constant {} to Drink", i)
            )
          )
        )
      },
    }
  }
}

//
// FullMeal
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FullMeal {
  pub meal: midlayer::Meal,
  pub dessert: midlayer::Dessert,
}

impl FullMeal {
  pub fn new(meal: midlayer::Meal, dessert: midlayer::Dessert) -> FullMeal {
    FullMeal {
      meal: meal,
      dessert: dessert,
    }
  }
  pub fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<FullMeal> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<midlayer::Meal> = None;
    let mut f_2: Option<midlayer::Dessert> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = midlayer::Meal::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        2 => {
          let val = midlayer::Dessert::read_from_in_protocol(i_prot)?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("FullMeal.meal", &f_1)?;
    verify_required_field_exists("FullMeal.dessert", &f_2)?;
    let ret = FullMeal {
      meal: f_1.expect("auto-generated code should have checked for presence of required fields"),
      dessert: f_2.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  pub fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("FullMeal");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("meal", TType::Struct, 1))?;
    self.meal.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("dessert", TType::Struct, 2))?;
    self.dessert.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// FullMealAndDrinks
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FullMealAndDrinks {
  pub full_meal: FullMeal,
  pub drink: Option<Drink>,
}

impl FullMealAndDrinks {
  pub fn new<F2>(full_meal: FullMeal, drink: F2) -> FullMealAndDrinks where F2: Into<Option<Drink>> {
    FullMealAndDrinks {
      full_meal: full_meal,
      drink: drink.into(),
    }
  }
  pub fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<FullMealAndDrinks> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<FullMeal> = None;
    let mut f_2: Option<Drink> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = FullMeal::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        2 => {
          let val = Drink::read_from_in_protocol(i_prot)?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("FullMealAndDrinks.full_meal", &f_1)?;
    let ret = FullMealAndDrinks {
      full_meal: f_1.expect("auto-generated code should have checked for presence of required fields"),
      drink: f_2,
    };
    Ok(ret)
  }
  pub fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("FullMealAndDrinks");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("fullMeal", TType::Struct, 1))?;
    self.full_meal.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.drink {
      o_prot.write_field_begin(&TFieldIdentifier::new("drink", TType::I32, 2))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// FullMealService service client
//

pub trait TFullMealServiceSyncClient : midlayer::TMealServiceSyncClient {
  fn full_meal(&mut self) -> thrift::Result<FullMeal>;
}

pub trait TFullMealServiceSyncClientMarker {}

pub struct FullMealServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  _i_prot: IP,
  _o_prot: OP,
  _sequence_number: i32,
}

impl <IP, OP> FullMealServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  pub fn new(input_protocol: IP, output_protocol: OP) -> FullMealServiceSyncClient<IP, OP> {
    FullMealServiceSyncClient { _i_prot: input_protocol, _o_prot: output_protocol, _sequence_number: 0 }
  }
}

impl <IP, OP> TThriftClient for FullMealServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  fn i_prot_mut(&mut self) -> &mut TInputProtocol { &mut self._i_prot }
  fn o_prot_mut(&mut self) -> &mut TOutputProtocol { &mut self._o_prot }
  fn sequence_number(&self) -> i32 { self._sequence_number }
  fn increment_sequence_number(&mut self) -> i32 { self._sequence_number += 1; self._sequence_number }
}

impl <IP, OP> TFullMealServiceSyncClientMarker for FullMealServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {}
impl <IP, OP> midlayer::TMealServiceSyncClientMarker for FullMealServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {}
impl <IP, OP> base_two::TRamenServiceSyncClientMarker for FullMealServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {}
impl <IP, OP> base_two::TNapkinServiceSyncClientMarker for FullMealServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {}

impl <C: TThriftClient + TFullMealServiceSyncClientMarker + midlayer::TMealServiceSyncClientMarker + base_two::TRamenServiceSyncClientMarker + base_two::TNapkinServiceSyncClientMarker> TFullMealServiceSyncClient for C {
  fn full_meal(&mut self) -> thrift::Result<FullMeal> {
    (
      {
        self.increment_sequence_number();
        let message_ident = TMessageIdentifier::new("fullMeal", TMessageType::Call, self.sequence_number());
        let call_args = FullMealArgs {  };
        self.o_prot_mut().write_message_begin(&message_ident)?;
        call_args.write_to_out_protocol(self.o_prot_mut())?;
        self.o_prot_mut().write_message_end()?;
        self.o_prot_mut().flush()
      }
    )?;
    {
      let message_ident = self.i_prot_mut().read_message_begin()?;
      verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
      verify_expected_service_call("fullMeal", &message_ident.name)?;
      if message_ident.message_type == TMessageType::Exception {
        let remote_error = thrift::Error::read_application_error_from_in_protocol(self.i_prot_mut())?;
        self.i_prot_mut().read_message_end()?;
        return Err(thrift::Error::Application(remote_error))
      }
      verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
      let result = FullMealResult::read_from_in_protocol(self.i_prot_mut())?;
      self.i_prot_mut().read_message_end()?;
      result.ok_or()
    }
  }
}

//
// FullMealService service processor
//

pub trait FullMealServiceSyncHandler : midlayer::MealServiceSyncHandler {
  fn handle_full_meal(&self) -> thrift::Result<FullMeal>;
}

pub struct FullMealServiceSyncProcessor<H: FullMealServiceSyncHandler> {
  handler: H,
}

impl <H: FullMealServiceSyncHandler> FullMealServiceSyncProcessor<H> {
  pub fn new(handler: H) -> FullMealServiceSyncProcessor<H> {
    FullMealServiceSyncProcessor {
      handler: handler,
    }
  }
  fn process_full_meal(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    TFullMealServiceProcessFunctions::process_full_meal(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
  fn process_meal(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    midlayer::TMealServiceProcessFunctions::process_meal(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
  fn process_ramen(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    base_two::TRamenServiceProcessFunctions::process_ramen(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
  fn process_napkin(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    base_two::TNapkinServiceProcessFunctions::process_napkin(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
}

pub struct TFullMealServiceProcessFunctions;

impl TFullMealServiceProcessFunctions {
  pub fn process_full_meal<H: FullMealServiceSyncHandler>(handler: &H, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let _ = FullMealArgs::read_from_in_protocol(i_prot)?;
    match handler.handle_full_meal() {
      Ok(handler_return) => {
        let message_ident = TMessageIdentifier::new("fullMeal", TMessageType::Reply, incoming_sequence_number);
        o_prot.write_message_begin(&message_ident)?;
        let ret = FullMealResult { result_value: Some(handler_return) };
        ret.write_to_out_protocol(o_prot)?;
        o_prot.write_message_end()?;
        o_prot.flush()
      },
      Err(e) => {
        match e {
          thrift::Error::Application(app_err) => {
            let message_ident = TMessageIdentifier::new("fullMeal", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&app_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
          _ => {
            let ret_err = {
              ApplicationError::new(
                ApplicationErrorKind::Unknown,
                e.description()
              )
            };
            let message_ident = TMessageIdentifier::new("fullMeal", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
        }
      },
    }
  }
}

impl <H: FullMealServiceSyncHandler> TProcessor for FullMealServiceSyncProcessor<H> {
  fn process(&self, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let message_ident = i_prot.read_message_begin()?;
    let res = match &*message_ident.name {
      "fullMeal" => {
        self.process_full_meal(message_ident.sequence_number, i_prot, o_prot)
      },
      "meal" => {
        self.process_meal(message_ident.sequence_number, i_prot, o_prot)
      },
      "ramen" => {
        self.process_ramen(message_ident.sequence_number, i_prot, o_prot)
      },
      "napkin" => {
        self.process_napkin(message_ident.sequence_number, i_prot, o_prot)
      },
      method => {
        Err(
          thrift::Error::Application(
            ApplicationError::new(
              ApplicationErrorKind::UnknownMethod,
              format!("unknown method {}", method)
            )
          )
        )
      },
    };
    thrift::server::handle_process_result(&message_ident, res, o_prot)
  }
}

//
// FullMealArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct FullMealArgs {
}

impl FullMealArgs {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<FullMealArgs> {
    i_prot.read_struct_begin()?;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = FullMealArgs {};
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("fullMeal_args");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// FullMealResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct FullMealResult {
  result_value: Option<FullMeal>,
}

impl FullMealResult {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<FullMealResult> {
    i_prot.read_struct_begin()?;
    let mut f_0: Option<FullMeal> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        0 => {
          let val = FullMeal::read_from_in_protocol(i_prot)?;
          f_0 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = FullMealResult {
      result_value: f_0,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("FullMealResult");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.result_value {
      o_prot.write_field_begin(&TFieldIdentifier::new("result_value", TType::Struct, 0))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
  fn ok_or(self) -> thrift::Result<FullMeal> {
    if self.result_value.is_some() {
      Ok(self.result_value.unwrap())
    } else {
      Err(
        thrift::Error::Application(
          ApplicationError::new(
            ApplicationErrorKind::MissingResult,
            "no result received for FullMeal"
          )
        )
      )
    }
  }
}

//
// FullMealAndDrinksService service client
//

pub trait TFullMealAndDrinksServiceSyncClient : TFullMealServiceSyncClient {
  fn full_meal_and_drinks(&mut self) -> thrift::Result<FullMealAndDrinks>;
}

pub trait TFullMealAndDrinksServiceSyncClientMarker {}

pub struct FullMealAndDrinksServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  _i_prot: IP,
  _o_prot: OP,
  _sequence_number: i32,
}

impl <IP, OP> FullMealAndDrinksServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  pub fn new(input_protocol: IP, output_protocol: OP) -> FullMealAndDrinksServiceSyncClient<IP, OP> {
    FullMealAndDrinksServiceSyncClient { _i_prot: input_protocol, _o_prot: output_protocol, _sequence_number: 0 }
  }
}

impl <IP, OP> TThriftClient for FullMealAndDrinksServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  fn i_prot_mut(&mut self) -> &mut TInputProtocol { &mut self._i_prot }
  fn o_prot_mut(&mut self) -> &mut TOutputProtocol { &mut self._o_prot }
  fn sequence_number(&self) -> i32 { self._sequence_number }
  fn increment_sequence_number(&mut self) -> i32 { self._sequence_number += 1; self._sequence_number }
}

impl <IP, OP> TFullMealAndDrinksServiceSyncClientMarker for FullMealAndDrinksServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {}
impl <IP, OP> TFullMealServiceSyncClientMarker for FullMealAndDrinksServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {}
impl <IP, OP> midlayer::TMealServiceSyncClientMarker for FullMealAndDrinksServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {}
impl <IP, OP> base_two::TRamenServiceSyncClientMarker for FullMealAndDrinksServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {}
impl <IP, OP> base_two::TNapkinServiceSyncClientMarker for FullMealAndDrinksServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {}

impl <C: TThriftClient + TFullMealAndDrinksServiceSyncClientMarker + TFullMealServiceSyncClientMarker + midlayer::TMealServiceSyncClientMarker + base_two::TRamenServiceSyncClientMarker + base_two::TNapkinServiceSyncClientMarker> TFullMealAndDrinksServiceSyncClient for C {
  fn full_meal_and_drinks(&mut self) -> thrift::Result<FullMealAndDrinks> {
    (
      {
        self.increment_sequence_number();
        let message_ident = TMessageIdentifier::new("fullMealAndDrinks", TMessageType::Call, self.sequence_number());
        let call_args = FullMealAndDrinksArgs {  };
        self.o_prot_mut().write_message_begin(&message_ident)?;
        call_args.write_to_out_protocol(self.o_prot_mut())?;
        self.o_prot_mut().write_message_end()?;
        self.o_prot_mut().flush()
      }
    )?;
    {
      let message_ident = self.i_prot_mut().read_message_begin()?;
      verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
      verify_expected_service_call("fullMealAndDrinks", &message_ident.name)?;
      if message_ident.message_type == TMessageType::Exception {
        let remote_error = thrift::Error::read_application_error_from_in_protocol(self.i_prot_mut())?;
        self.i_prot_mut().read_message_end()?;
        return Err(thrift::Error::Application(remote_error))
      }
      verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
      let result = FullMealAndDrinksResult::read_from_in_protocol(self.i_prot_mut())?;
      self.i_prot_mut().read_message_end()?;
      result.ok_or()
    }
  }
}

//
// FullMealAndDrinksService service processor
//

pub trait FullMealAndDrinksServiceSyncHandler : FullMealServiceSyncHandler {
  fn handle_full_meal_and_drinks(&self) -> thrift::Result<FullMealAndDrinks>;
}

pub struct FullMealAndDrinksServiceSyncProcessor<H: FullMealAndDrinksServiceSyncHandler> {
  handler: H,
}

impl <H: FullMealAndDrinksServiceSyncHandler> FullMealAndDrinksServiceSyncProcessor<H> {
  pub fn new(handler: H) -> FullMealAndDrinksServiceSyncProcessor<H> {
    FullMealAndDrinksServiceSyncProcessor {
      handler: handler,
    }
  }
  fn process_full_meal_and_drinks(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    TFullMealAndDrinksServiceProcessFunctions::process_full_meal_and_drinks(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
  fn process_full_meal(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    TFullMealServiceProcessFunctions::process_full_meal(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
  fn process_meal(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    midlayer::TMealServiceProcessFunctions::process_meal(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
  fn process_ramen(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    base_two::TRamenServiceProcessFunctions::process_ramen(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
  fn process_napkin(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    base_two::TNapkinServiceProcessFunctions::process_napkin(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
}

pub struct TFullMealAndDrinksServiceProcessFunctions;

impl TFullMealAndDrinksServiceProcessFunctions {
  pub fn process_full_meal_and_drinks<H: FullMealAndDrinksServiceSyncHandler>(handler: &H, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let _ = FullMealAndDrinksArgs::read_from_in_protocol(i_prot)?;
    match handler.handle_full_meal_and_drinks() {
      Ok(handler_return) => {
        let message_ident = TMessageIdentifier::new("fullMealAndDrinks", TMessageType::Reply, incoming_sequence_number);
        o_prot.write_message_begin(&message_ident)?;
        let ret = FullMealAndDrinksResult { result_value: Some(handler_return) };
        ret.write_to_out_protocol(o_prot)?;
        o_prot.write_message_end()?;
        o_prot.flush()
      },
      Err(e) => {
        match e {
          thrift::Error::Application(app_err) => {
            let message_ident = TMessageIdentifier::new("fullMealAndDrinks", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&app_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
          _ => {
            let ret_err = {
              ApplicationError::new(
                ApplicationErrorKind::Unknown,
                e.description()
              )
            };
            let message_ident = TMessageIdentifier::new("fullMealAndDrinks", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
        }
      },
    }
  }
}

impl <H: FullMealAndDrinksServiceSyncHandler> TProcessor for FullMealAndDrinksServiceSyncProcessor<H> {
  fn process(&self, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let message_ident = i_prot.read_message_begin()?;
    let res = match &*message_ident.name {
      "fullMealAndDrinks" => {
        self.process_full_meal_and_drinks(message_ident.sequence_number, i_prot, o_prot)
      },
      "fullMeal" => {
        self.process_full_meal(message_ident.sequence_number, i_prot, o_prot)
      },
      "meal" => {
        self.process_meal(message_ident.sequence_number, i_prot, o_prot)
      },
      "ramen" => {
        self.process_ramen(message_ident.sequence_number, i_prot, o_prot)
      },
      "napkin" => {
        self.process_napkin(message_ident.sequence_number, i_prot, o_prot)
      },
      method => {
        Err(
          thrift::Error::Application(
            ApplicationError::new(
              ApplicationErrorKind::UnknownMethod,
              format!("unknown method {}", method)
            )
          )
        )
      },
    };
    thrift::server::handle_process_result(&message_ident, res, o_prot)
  }
}

//
// FullMealAndDrinksArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct FullMealAndDrinksArgs {
}

impl FullMealAndDrinksArgs {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<FullMealAndDrinksArgs> {
    i_prot.read_struct_begin()?;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = FullMealAndDrinksArgs {};
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("fullMealAndDrinks_args");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// FullMealAndDrinksResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct FullMealAndDrinksResult {
  result_value: Option<FullMealAndDrinks>,
}

impl FullMealAndDrinksResult {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<FullMealAndDrinksResult> {
    i_prot.read_struct_begin()?;
    let mut f_0: Option<FullMealAndDrinks> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        0 => {
          let val = FullMealAndDrinks::read_from_in_protocol(i_prot)?;
          f_0 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = FullMealAndDrinksResult {
      result_value: f_0,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("FullMealAndDrinksResult");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.result_value {
      o_prot.write_field_begin(&TFieldIdentifier::new("result_value", TType::Struct, 0))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
  fn ok_or(self) -> thrift::Result<FullMealAndDrinks> {
    if self.result_value.is_some() {
      Ok(self.result_value.unwrap())
    } else {
      Err(
        thrift::Error::Application(
          ApplicationError::new(
            ApplicationErrorKind::MissingResult,
            "no result received for FullMealAndDrinks"
          )
        )
      )
    }
  }
}

