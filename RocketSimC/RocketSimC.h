#pragma once
#include <cstdint>
#ifndef ROCKETSIMC_H
#define ROCKETSIMC_H

#ifndef EXPORT
	#if defined(_MSC_VER)
		#define EXPORT __declspec(dllexport)
	#elif defined(__GNUC__)
		#define EXPORT __attribute__((visibility("default")))
	#endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

	EXPORT typedef struct CarStateSnapshot {
		Team team;
		uint32_t id;

		// Position in world space
		float pos_x, pos_y, pos_z;
		float yaw, pitch, roll;

		// Linear velocity
		float vel_x, vel_y, vel_z;

		// Angular velocity (axis-angle)
		float angVel_x, angVel_y, angVel_z;

		bool isOnGround;
		bool hasJumped, hasDoubleJumped, hasFlipped; 
		float lastRelDodgeTorque_x, lastRelDodgeTorque_y, lastRelDodgeTorque_z;

		// Active during a jump or flip
		float jumpTimer, flipTimer;

		// True during a jump (not double jumps or a flip)
		bool isJumping;

		// Time spent in the air once !isJumping
		float airTimeSinceJump;

		// Goes from 0 to 100
		float boost;

		// This is a state variable due to the supersonic maintain time (see RLConst.h)
		bool isSupersonic;

		// This is a state variable due to the rise/fall rate of handbrake inputs (see RLConst.h)
		float handbrakeVal;
	} CarStateSnapshot;

	EXPORT typedef struct BallStateSnapshot {
		// Position in world space
		float pos_x, pos_y, pos_z;

		// Linear velocity
		float vel_x, vel_y, vel_z;

		// Angular velocity (axis-angle)
		float angVel_x, angVel_y, angVel_z;
	} BallStateSnapshot;

	//typedef void* Arena;
	typedef uint32_t CarID;

	EXPORT Arena* CreateArena(int gameMode);
	EXPORT void DestroyArena(Arena* arena);
	EXPORT CarID AddCar(Arena* arena, int team);
	EXPORT bool RemoveCar(Arena* arena, CarID carId);
	EXPORT bool Controls(Arena* arena, CarID carId, float throttle, float steer, float pitch, float yaw, float roll, bool boost, bool jump, bool handbrake);
	EXPORT bool GetCarState(Arena* arena, CarID carId, CarStateSnapshot* target);
	EXPORT bool SetCarState(Arena* arena, CarID carId, CarStateSnapshot* target);
	EXPORT bool GetBallState(Arena* arena, BallStateSnapshot* target);
	EXPORT bool SetBallState(Arena* arena, BallStateSnapshot* target);
	EXPORT bool Step(Arena* arena, int ticksToSimulate);

#ifdef __cplusplus
}
#endif

#endif // ROCKETSIMC_H