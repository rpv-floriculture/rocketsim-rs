// RocketSimC.cpp : Defines the functions for the static library.
//

#include "pch.h"
#include "framework.h"
#include "Sim/Arena/Arena.h"
#include "RocketSimC.h"

#if defined(_MSC_VER)
	#define EXPORT __declspec(dllexport)
#elif defined(__GNUC__)
	#define EXPORT __attribute__((visibility("default")))
#endif

#ifdef __cplusplus
extern "C" {
#endif

		EXPORT Arena* CreateArena(int gameMode) {
			auto arena = new Arena((GameMode)gameMode);
			if (arena == nullptr) {
				return nullptr;
			}
			return arena;
		}

		EXPORT void DestroyArena(Arena* arena) {
			delete arena;
		}

		EXPORT CarID AddCar(Arena* arena, int team) {
			if (arena == nullptr) {
				return 0;
			}
			auto car = arena->AddCar((Team)team, CAR_CONFIG_OCTANE);
			if (car == nullptr) {
				return 0;
			}
			return car->id;
		}

		EXPORT bool RemoveCar(Arena* arena, CarID carId) {
			if (arena == nullptr) {
				return 0;
			}
			auto car = arena->GetCarFromID(carId);
			if (car == nullptr) {
				return false;
			}
			return arena->RemoveCar(car);
		}

		EXPORT bool Controls(Arena* arena, CarID carId, float throttle, float steer, float pitch, float yaw, float roll, bool boost, bool jump, bool handbrake) {
			if (arena == nullptr) {
				return false;
			}
			auto car = arena->GetCarFromID(carId);
			if (car == nullptr) {
				return false;
			}
			car->controls.throttle = throttle;
			car->controls.steer = steer;
			car->controls.pitch = pitch;
			car->controls.yaw = yaw;
			car->controls.roll = roll;
			car->controls.boost = boost;
			car->controls.jump = jump;
			car->controls.handbrake = handbrake;
			return true;
		}


		EXPORT bool GetCarState(Arena* arena, CarID carId, CarStateSnapshot* target) {
			if (arena == nullptr) {
				return false;
			}
			auto car = arena->GetCarFromID(carId);
			if (car == nullptr) {
				return false;
			}
			auto carState = car->GetState();
			target->team = car->team;
			target->id = car->id;
			target->pos_x = carState.pos.x();
			target->pos_y = carState.pos.y();
			target->pos_z = carState.pos.z();
			target->yaw = carState.angles.yaw;
			target->pitch = carState.angles.pitch;
			target->roll = carState.angles.roll;
			target->vel_x = carState.vel.x();
			target->vel_y = carState.vel.y();
			target->vel_z = carState.vel.z();
			target->angVel_x = carState.angVel.x();
			target->angVel_y = carState.angVel.y();
			target->angVel_z = carState.angVel.z();
			target->isOnGround = carState.isOnGround;
			target->hasJumped = carState.hasJumped;
			target->hasDoubleJumped = carState.hasDoubleJumped;
			target->hasFlipped = carState.hasFlipped;
			target->lastRelDodgeTorque_x = carState.lastRelDodgeTorque.x();
			target->lastRelDodgeTorque_y = carState.lastRelDodgeTorque.y();
			target->lastRelDodgeTorque_z = carState.lastRelDodgeTorque.z();
			target->jumpTimer = carState.jumpTimer;
			target->flipTimer = carState.flipTimer;
			target->isJumping = carState.isJumping;
			target->airTimeSinceJump = carState.airTimeSinceJump;
			target->boost = carState.boost;
			target->isSupersonic = carState.isSupersonic;
			target->handbrakeVal = carState.handbrakeVal;
			return true;
		}

		EXPORT bool SetCarState(Arena* arena, CarID carId, CarStateSnapshot* source) {
			if (arena == nullptr) {
				return false;
			}
			auto car = arena->GetCarFromID(carId);
			if (car == nullptr) {
				return false;
			}
			auto target = car->GetState();
			target.pos.setX(source->pos_x);
			target.pos.setY(source->pos_y);
			target.pos.setZ(source->pos_z);
			target.angles.yaw = source->yaw;
			target.angles.pitch = source->pitch;
			target.angles.roll = source->roll;
			target.vel.setX(source->vel_x);
			target.vel.setY(source->vel_y);
			target.vel.setZ(source->vel_z);
			target.angVel.setX(source->angVel_x);
			target.angVel.setY(source->angVel_y);
			target.angVel.setZ(source->angVel_z);
			target.isOnGround = source->isOnGround;
			target.hasJumped = source->hasJumped;
			target.hasDoubleJumped = source->hasDoubleJumped;
			target.hasFlipped = source->hasFlipped;
			target.lastRelDodgeTorque.setX(source->lastRelDodgeTorque_x);
			target.lastRelDodgeTorque.setY(source->lastRelDodgeTorque_y);
			target.lastRelDodgeTorque.setZ(source->lastRelDodgeTorque_z);
			target.jumpTimer = source->jumpTimer;
			target.flipTimer = source->flipTimer;
			target.isJumping = source->isJumping;
			target.airTimeSinceJump = source->airTimeSinceJump;
			target.boost = source->boost;
			target.isSupersonic = source->isSupersonic;
			target.handbrakeVal = source->handbrakeVal;
			car->SetState(target);
			return true;
		}

		EXPORT bool GetBallState(Arena* arena, BallStateSnapshot* target) {
			if (arena == nullptr) {
				return false;
			}
			auto ball = arena->ball;
			if (ball == nullptr) {
				return false;
			}
			auto ballstate = ball->GetState();
			target->pos_x = ballstate.pos.x();
			target->pos_y = ballstate.pos.y();
			target->pos_z = ballstate.pos.z();
			target->vel_x = ballstate.vel.x();
			target->vel_y = ballstate.vel.y();
			target->vel_z = ballstate.vel.z();
			target->angVel_x = ballstate.angVel.x();
			target->angVel_y = ballstate.angVel.y();
			target->angVel_z = ballstate.angVel.z();
			return true;
		}

		EXPORT bool SetBallState(Arena* arena, BallStateSnapshot* source) {
			if (arena == nullptr) {
				return false;
			}
			auto ball = arena->ball;
			if (ball == nullptr) {
				return false;
			}
			auto target = ball->GetState();
			target.pos.setX(source->pos_x);
			target.pos.setY(source->pos_y);
			target.pos.setZ(source->pos_z);
			target.vel.setX(source->vel_x);
			target.vel.setY(source->vel_y);
			target.vel.setZ(source->vel_z);
			target.angVel.setX(source->angVel_x);
			target.angVel.setY(source->angVel_y);
			target.angVel.setZ(source->angVel_z);
			ball->SetState(target);
			return true;
		}

		EXPORT bool Step(Arena* arena, int ticksToSimulate) {
			if (arena == nullptr) {
				return false;
			}
			arena->Step(ticksToSimulate);
			return true;
		}

#ifdef __cplusplus
	}
#endif