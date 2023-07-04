// Copyright (c) 2017 Pieter Wuille
// Distributed under the MIT software license, see the accompanying
// file LICENSE or http://www.opensource.org/licenses/mit-license.php.

#include <test/setup_common.h>
#include <boost/test/unit_test.hpp>

BOOST_FIXTURE_TEST_SUITE(empty_tests, BasicTestingSetup, *boost::unit_test::disabled())


BOOST_AUTO_TEST_CASE(empty_test_case_1)
{
    BOOST_TEST_MESSAGE("Hello world!");
}

BOOST_AUTO_TEST_SUITE_END()
