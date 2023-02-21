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
			target->pos = carState.pos;
			target->rot = carState.rot.transpose();
			target->vel = carState.vel;
			target->angVel = carState.angVel;
			target->isOnGround = carState.isOnGround;
			target->hasJumped = carState.hasJumped;
			target->hasDoubleJumped = carState.hasDoubleJumped;
			target->hasFlipped = carState.hasFlipped;
			target->lastRelDodgeTorque = carState.lastRelDodgeTorque;
			target->jumpTimer = carState.jumpTimer;
			target->flipTimer = carState.flipTimer;
			target->isJumping = carState.isJumping;
			target->airTimeSinceJump = carState.airTimeSinceJump;
			target->boost = carState.boost;
			target->timeSpentBoosting = carState.timeSpentBoosting;
			target->isSupersonic = carState.isSupersonic;
			target->supersonicTime = carState.supersonicTime;
			target->handbrakeVal = carState.handbrakeVal;
			target->isAutoFlipping = carState.isAutoFlipping;
			target->autoFlipTimer = carState.autoFlipTimer;
			target->autoFlipTorqueScale = carState.autoFlipTorqueScale;
			target->hasContact = carState.worldContact.hasContact;
			target->contactNormal = carState.worldContact.contactNormal;
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
			target.pos = source->pos;
			target.rot = source->rot.transpose();
			target.vel = source->vel;
			target.angVel = source->angVel;
			target.isOnGround = source->isOnGround;
			target.hasJumped = source->hasJumped;
			target.hasDoubleJumped = source->hasDoubleJumped;
			target.hasFlipped = source->hasFlipped;
			target.lastRelDodgeTorque = source->lastRelDodgeTorque;
			target.jumpTimer = source->jumpTimer;
			target.flipTimer = source->flipTimer;
			target.isJumping = source->isJumping;
			target.airTimeSinceJump = source->airTimeSinceJump;
			target.boost = source->boost;
			target.timeSpentBoosting = source->timeSpentBoosting;
			target.isSupersonic = source->isSupersonic;
			target.handbrakeVal = source->handbrakeVal;
			target.isAutoFlipping = source->isAutoFlipping;
			target.autoFlipTimer = source->autoFlipTimer;
			target.autoFlipTorqueScale = source->autoFlipTorqueScale;
			target.worldContact.hasContact = source->hasContact;
			target.worldContact.contactNormal = source->contactNormal;
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
			target->pos = ballstate.pos;
			target->vel = ballstate.vel;
			target->angVel = ballstate.angVel;
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
			target.pos = source->pos;
			target.vel = source->vel;
			target.angVel = source->angVel;
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