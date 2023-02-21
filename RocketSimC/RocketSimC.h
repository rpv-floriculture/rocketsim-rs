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
		btVector3 pos;

		// Rotation
		btMatrix3x3 rot;

		// Linear velocity
		btVector3 vel;

		// Angular velocity (axis-angle)
		btVector3 angVel;

		bool isOnGround;
		bool hasJumped, hasDoubleJumped, hasFlipped; 
		btVector3 lastRelDodgeTorque;

		// Active during a jump or flip
		float jumpTimer, flipTimer;

		// True during a jump (not double jumps or a flip)
		bool isJumping;

		// Time spent in the air once !isJumping
		float airTimeSinceJump;

		// Goes from 0 to 100
		float boost;

		// Added to replicate minimum boosting time
		// NOTE: Will be used even when we have no boost
		float timeSpentBoosting;

		// This is a state variable due to the supersonic maintain time (see RLConst.h)
		bool isSupersonic;

		// Time spent supersonic, for checking with the supersonic maintain time (see RLConst.h)
		float supersonicTime;

		// This is a state variable due to the rise/fall rate of handbrake inputs (see RLConst.h)
		float handbrakeVal;

		bool isAutoFlipping;
		float autoFlipTimer; // Counts down when auto-flipping
		float autoFlipTorqueScale;

		bool hasContact;
		btVector3 contactNormal;
	} CarStateSnapshot;

	EXPORT typedef struct BallStateSnapshot {
		// Position in world space
		btVector3 pos;

		// Linear velocity
		btVector3 vel;

		// Angular velocity (axis-angle)
		btVector3 angVel;
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